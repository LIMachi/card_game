use crate::game::events::{GameEvent, GameEvents, GameEventsPlugin};
use crate::prelude::*;
use crate::utils::ray_caster::RayCasterPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};

mod cards;
mod game;
mod players;
mod stacks;
mod states;
mod utils;

pub mod prelude {
    pub use super::cards::prelude::*;
    pub use super::stacks::*;
    pub use super::states::app::AppStates;
    pub use super::utils::filter_enum::FilterEnumInserter;
    pub use super::utils::ray_caster::RayCaster;
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

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RayCasterPlugin,
            cards::CardsPlugin,
            players::PlayerPlugin,
            StacksPlugin,
            states::StatesPlugin,
            GameEventsPlugin,
            utils::debug::DebugPlugin,
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
