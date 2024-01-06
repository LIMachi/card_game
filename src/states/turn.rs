use bevy::prelude::*;

#[derive(States, Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Reflect)]
#[reflect(Default)]
pub enum TurnStates {
    #[default]
    Setup,
    PlayerTurn(u8),
    PlayerCleanup(u8),
}
