use crate::cards::spawn::spawn_card;
use crate::states::app::AppStates;
use bevy::prelude::*;

pub mod actions;
pub mod assets;
pub mod components;
pub mod spawn;

pub struct CardsPlugin;

pub mod prelude {
    pub use super::{
        actions::{
            ComboBlob, ComboMachineCult, ComboStarEmpire, ComboTradeFederation, OnPlay, OnScrap,
        },
        components::prelude::*,
        spawn::SpawnCard,
    };
}

impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            actions::GameActionsPlugin,
            assets::SetPlugin,
            components::CardComponentsPlugin,
        ))
        .add_systems(PreUpdate, spawn_card.run_if(in_state(AppStates::Playing)));
    }
}
