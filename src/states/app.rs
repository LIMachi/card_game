use bevy::prelude::*;

#[derive(States, Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Reflect)]
#[reflect(Default)]
pub enum AppStates {
    #[default]
    Loading,
    MainMenu,
    Playing,
}
