pub mod ui;

use crate::players::ui::CountersUIPlugin;
use crate::prelude::{CardOwners, FilterEnumInserter};
use bevy::prelude::*;

pub const MAXIMUM_PLAYERS: usize = 2;

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct LocalPlayer(pub u8);

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Player<const ID: u8>;

pub trait PlayerCounter {
    fn get_value(&self) -> i32;
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PlayerLife(pub i32);

impl PlayerCounter for PlayerLife {
    fn get_value(&self) -> i32 {
        self.0
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PlayerAttack(pub i32);

impl PlayerCounter for PlayerAttack {
    fn get_value(&self) -> i32 {
        self.0
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PlayerEconomy(pub i32);

impl PlayerCounter for PlayerEconomy {
    fn get_value(&self) -> i32 {
        self.0
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PlayerActionTracker {}

pub fn spawn_counters(mut commands: Commands) {
    CardOwners::Player(0).insert(&mut commands.spawn((
        Name::new(format!("Player 0 counters")),
        PlayerLife(50),
        PlayerAttack(0),
        PlayerEconomy(0),
    )));
    CardOwners::Player(1).insert(&mut commands.spawn((
        Name::new(format!("Player 1 counters")),
        PlayerLife(50),
        PlayerAttack(0),
        PlayerEconomy(0),
    )));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LocalPlayer>()
            .init_resource::<LocalPlayer>()
            .register_type::<PlayerLife>()
            .register_type::<PlayerAttack>()
            .register_type::<PlayerEconomy>()
            .register_type::<PlayerActionTracker>()
            .register_type::<Player<0>>()
            .register_type::<Player<1>>()
            .add_systems(Startup, spawn_counters)
            .add_plugins(CountersUIPlugin);
    }
}
