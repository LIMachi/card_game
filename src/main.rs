use crate::cards::assets::{Deck, LoadedSet};
use crate::game::events::{GameEvent, GameEvents, GameEventsPlugin};
use crate::prelude::*;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};

mod cards;
mod debug;
mod game;
mod players;
mod ray_caster;
mod stacks;
mod states;
mod utils;

pub mod prelude {
    pub use super::cards::prelude::*;
    pub use super::ray_caster::RayCaster;
    pub use super::stacks::*;
    pub use super::states::app::AppStates;
    pub use super::utils::filter_enum::FilterEnumInserter;
    pub use bevy::prelude::*;
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., -50., 0.0).looking_at(Vec3::ZERO, Vec3::Z),
            projection: Projection::from(PerspectiveProjection {
                ..Default::default()
            }),
            ..Default::default()
        },
        RayCaster::default(),
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

// pub fn spawn_decks(mut commands: Commands, set: Res<LoadedSet>, decks: Res<Assets<Deck>>) {
//     if let Some(deck) = decks.get(&set.market_deck) {
//         let mut index = 0;
//         for (qty, name) in &deck.0 {
//             for _ in 0..*qty {
//                 let mut ec = commands.spawn((
//                     CardIndex(index),
//                     SpawnCard(name.clone()),
//                     SpatialBundle::default(),
//                     Name::new(name.clone()),
//                     StartTransition {
//                         owner: CardOwners::Market,
//                         stack: Stacks::MarketDeck,
//                         index: CardIndex(index),
//                         visibility: CardVisibility::Visible,
//                         length: 0.0,
//                     },
//                 ));
//                 CardOwners::Market.insert(&mut ec);
//                 Stacks::MarketDeck.insert(&mut ec);
//                 index += 1;
//             }
//         }
//     }
//     if let Some(deck) = decks.get(&set.player_deck) {
//         let mut index = 0;
//         for (qty, name) in &deck.0 {
//             for _ in 0..*qty {
//                 let mut ec = commands.spawn((
//                     CardIndex(index),
//                     SpawnCard(name.clone()),
//                     SpatialBundle::from_transform(Transform::from_xyz(
//                         30.,
//                         CARD_DEPTH * index as f32,
//                         -15.,
//                     )),
//                     Name::new(name.clone()),
//                 ));
//                 CardOwners::Player(0).insert(&mut ec);
//                 Stacks::PlayerDeck.insert(&mut ec);
//                 let mut ec = commands.spawn((
//                     CardIndex(index),
//                     SpawnCard(name.clone()),
//                     SpatialBundle::from_transform(Transform::from_xyz(
//                         30.,
//                         CARD_DEPTH * index as f32,
//                         15.,
//                     )),
//                     Name::new(name.clone()),
//                 ));
//                 CardOwners::Player(1).insert(&mut ec);
//                 Stacks::PlayerDeck.insert(&mut ec);
//                 index += 1;
//             }
//         }
//     }
//     if let Some(deck) = decks.get(&set.joker_deck) {
//         let mut index = 0;
//         for (qty, name) in &deck.0 {
//             for _ in 0..*qty {
//                 let mut ec = commands.spawn((
//                     CardIndex(index),
//                     SpawnCard(name.clone()),
//                     SpatialBundle::from_transform(
//                         Transform::from_xyz(-25., CARD_DEPTH * index as f32, 0.)
//                             .with_rotation(Quat::from_axis_angle(Vec3::Z, 180f32.to_radians())),
//                     ),
//                     Name::new(name.clone()),
//                 ));
//                 CardOwners::Market.insert(&mut ec);
//                 Stacks::JokerDeck.insert(&mut ec);
//                 index += 1;
//             }
//         }
//     }
// }

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            ray_caster::RayCasterPlugin,
            cards::CardsPlugin,
            players::PlayerPlugin,
            stacks::StacksPlugin,
            states::StatesPlugin,
            GameEventsPlugin,
            debug::DebugPlugin,
        ))
        .add_systems(Startup, (spawn_camera, spawn_light))
        .add_systems(
            OnEnter(AppStates::Playing),
            |mut events: ResMut<GameEvent>| {
                events.push(GameEvents::Setup {
                    seed: 0,
                    set: 0,
                    players: 2,
                    starting_player: 0,
                });
            },
        )
        .run();
}
