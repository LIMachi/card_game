use bevy::prelude::*;

pub const MAXIMUM_PLAYERS: usize = 2;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Player<const ID: u8>;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PlayerLife(pub i32);

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PlayerAttack(pub i32);

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PlayerEconomy(pub i32);

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PlayerActionTracker {}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerLife>()
            .register_type::<PlayerAttack>()
            .register_type::<PlayerEconomy>()
            .register_type::<PlayerActionTracker>()
            .register_type::<Player<0>>()
            .register_type::<Player<1>>();
    }
}
