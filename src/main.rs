use crate::cards::assets::{Deck, LoadedSet};
use crate::cards::prelude::*;
use crate::fps::{fps_counter_showhide, fps_text_update_system, setup_fps_counter};
use crate::stacks::Stacks;
use crate::states::app::AppStates;
use crate::utils::filter_enum::FilterEnumInserter;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod cards;
mod fps;
mod players;
mod stacks;
mod states;
mod utils;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., -50., 0.0).looking_at(Vec3::ZERO, Vec3::Z),
            projection: Projection::from(PerspectiveProjection {
                ..Default::default()
            }),
            ..Default::default()
        },
        // RayCaster::default(),
    ));
}

pub fn spawn_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(0., -1000., 0.).looking_at(Vec3::ZERO, Vec3::Z),
        directional_light: DirectionalLight {
            color: Default::default(),
            illuminance: 4000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });
}

#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct Debug(pub bool);

pub fn spawn_decks(mut commands: Commands, set: Res<LoadedSet>, decks: Res<Assets<Deck>>) {
    if let Some(deck) = decks.get(&set.market_deck) {
        let mut index = 0;
        for (qty, name) in &deck.0 {
            for _ in 0..*qty {
                let mut ec = commands.spawn((
                    CardIndex(index),
                    SpawnCard(name.clone()),
                    SpatialBundle::default(),
                    Name::new(name.clone()),
                ));
                CardOwners::Market.insert(&mut ec);
                Stacks::MarketDeck.insert(&mut ec);
                index += 1;
            }
        }
    }
    if let Some(deck) = decks.get(&set.player_deck) {
        let mut index = 0;
        for (qty, name) in &deck.0 {
            for _ in 0..*qty {
                let mut ec = commands.spawn((
                    CardIndex(index),
                    SpawnCard(name.clone()),
                    SpatialBundle::default(),
                    Name::new(name.clone()),
                ));
                CardOwners::Player(0).insert(&mut ec);
                Stacks::PlayerDeck.insert(&mut ec);
                let mut ec = commands.spawn((
                    CardIndex(index),
                    SpawnCard(name.clone()),
                    SpatialBundle::default(),
                    Name::new(name.clone()),
                ));
                CardOwners::Player(1).insert(&mut ec);
                Stacks::PlayerDeck.insert(&mut ec);
                index += 1;
            }
        }
    }
    if let Some(deck) = decks.get(&set.joker_deck) {
        let mut index = 0;
        for (qty, name) in &deck.0 {
            for _ in 0..*qty {
                let mut ec = commands.spawn((
                    CardIndex(index),
                    SpawnCard(name.clone()),
                    SpatialBundle::default(),
                    Name::new(name.clone()),
                ));
                CardOwners::Market.insert(&mut ec);
                Stacks::JokerDeck.insert(&mut ec);
                index += 1;
            }
        }
    }
}

fn main() {
    App::new()
        .insert_resource(Debug(true))
        .add_systems(Update, |k: Res<Input<KeyCode>>, mut d: ResMut<Debug>| {
            if k.just_pressed(KeyCode::Apps) {
                d.0 ^= true;
            }
        })
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::default().run_if(|d: Res<Debug>| d.0),
            FrameTimeDiagnosticsPlugin::default(),
            cards::CardsPlugin,
            players::PlayerPlugin,
            stacks::StacksPlugin,
            states::StatesPlugin,
        ))
        .add_systems(Startup, (spawn_camera, spawn_light, setup_fps_counter))
        .add_systems(OnEnter(AppStates::Playing), spawn_decks)
        .add_systems(Update, (fps_text_update_system, fps_counter_showhide))
        .run();
}
