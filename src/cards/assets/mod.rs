pub mod prepare_models;
pub mod serializer;

use crate::cards::actions::ActionSet;
use crate::cards::assets::prepare_models::prepare_models;
use crate::cards::assets::serializer::{CardLoadder, DeckLoadder};
use crate::cards::components::factions::CardFactions;
use crate::cards::components::kinds::CardKinds;
use crate::states::app::AppStates;
use bevy::asset::LoadedFolder;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;

#[derive(Asset, Debug, Deserialize, Default, Reflect, Clone)]
#[reflect(Debug)]
pub struct Card {
    pub kind: CardKinds,
    pub factions: Vec<CardFactions>,
    pub cost: i32,
    pub play: ActionSet,
    pub scrap: ActionSet,
    pub combo: HashMap<CardFactions, ActionSet>,
}

#[derive(Asset, Debug, Default, Reflect, Clone)]
#[reflect(Debug)]
pub struct Deck(pub Vec<(u32, String)>);

#[derive(Resource, Default, Debug, Reflect, Clone)]
#[reflect(Resource)]
pub struct LoadedSet {
    pub ready: bool,
    pub market_deck: Handle<Deck>,
    pub player_deck: Handle<Deck>,
    pub joker_deck: Handle<Deck>,
    pub cards: HashMap<String, Handle<Card>>,
}

fn load_decks(mut set: ResMut<LoadedSet>, asset_server: Res<AssetServer>) {
    //TODO: add selection of set/game mode
    let set_name = "debug_bases";
    set.market_deck = asset_server.load(format!("sets/{set_name}/market.deck.ron"));
    set.player_deck = asset_server.load(format!("sets/{set_name}/player.deck.ron"));
    set.joker_deck = asset_server.load(format!("sets/{set_name}/explorer.deck.ron"));
}

fn load_cards(
    mut once: Local<bool>,
    mut set: ResMut<LoadedSet>,
    asset_server: Res<AssetServer>,
    decks: Res<Assets<Deck>>,
) {
    if !*once {
        if let Some(market) = decks.get(&set.market_deck) {
            if let Some(player) = decks.get(&set.player_deck) {
                if let Some(explorer) = decks.get(&set.joker_deck) {
                    for (_, name) in market
                        .0
                        .iter()
                        .chain(player.0.iter())
                        .chain(explorer.0.iter())
                    {
                        if set.cards.contains_key(name) {
                            continue;
                        }
                        set.cards.insert(
                            name.clone(),
                            asset_server.load(format!("cards/{}.card.ron", name)),
                        );
                    }
                    set.ready = true;
                    *once = true;
                }
            }
        }
    }
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct RawModel {
    pub gltf: Handle<Gltf>,
    pub ships: Handle<LoadedFolder>,
    pub bases: Handle<LoadedFolder>,
}

pub fn load_raw_models(assets: Res<AssetServer>, mut raw: ResMut<RawModel>) {
    raw.gltf = assets.load("card.gltf");
    raw.ships = assets.load_folder("textures/ships");
    raw.bases = assets.load_folder("textures/bases");
}

pub fn raw_models_ready(raw: Res<RawModel>, assets: Res<AssetServer>) -> bool {
    assets.is_loaded_with_dependencies(&raw.gltf)
        && assets.is_loaded_with_dependencies(&raw.ships)
        && assets.is_loaded_with_dependencies(&raw.bases)
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct LoadedModels {
    pub ready: bool,
    pub back_mesh: Handle<Mesh>,
    pub back_material: Handle<StandardMaterial>,
    pub side_mesh: Handle<Mesh>,
    pub side_material: Handle<StandardMaterial>,
    pub front_ship_mesh: Handle<Mesh>,
    pub front_base_mesh: Handle<Mesh>,
    pub front_materials: HashMap<String, Handle<StandardMaterial>>,
}

pub fn finished_loadding(
    mut state: ResMut<NextState<AppStates>>,
    models: Res<LoadedModels>,
    set: Res<LoadedSet>,
) {
    if models.ready && set.ready {
        state.set(AppStates::Playing); //TODO: should switch to main menu instead
    }
}

pub struct SetPlugin;

impl Plugin for SetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RawModel>()
            .init_resource::<RawModel>()
            .register_type::<LoadedSet>()
            .init_resource::<LoadedSet>()
            .register_type::<LoadedModels>()
            .init_resource::<LoadedModels>()
            .init_asset::<Card>()
            .register_asset_reflect::<Card>()
            .init_asset_loader::<CardLoadder>()
            .init_asset::<Deck>()
            .register_asset_reflect::<Deck>()
            .init_asset_loader::<DeckLoadder>()
            .add_systems(
                Startup,
                (load_raw_models, load_decks).run_if(in_state(AppStates::Loading)),
            )
            .add_systems(
                First,
                (
                    load_cards,
                    prepare_models.run_if(raw_models_ready),
                    finished_loadding,
                )
                    .run_if(in_state(AppStates::Loading)),
            );
    }
}
