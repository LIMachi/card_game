use crate::cards::spawn::spawn_card;
use crate::states::app::AppStates;
use bevy::prelude::*;

pub mod actions;
pub mod assets;
pub mod components;
pub mod spawn;
pub mod transition;
mod transitions_transforms;

pub const CARD_DEPTH: f32 = 2. / 70.;
pub const CARD_WIDTH: f32 = 6.3;
pub const CARD_HEIGHT: f32 = 8.8;

pub struct CardsPlugin;

pub mod prelude {
    pub use super::{
        actions::{
            ActionSet, ComboBlob, ComboMachineCult, ComboStarEmpire, ComboTradeFederation, OnPlay,
            OnScrap,
        },
        components::prelude::*,
        spawn::SpawnCard,
        transition::{CardTransition, StartTransition, TransitionTransforms},
        CARD_DEPTH, CARD_HEIGHT, CARD_WIDTH,
    };
}

impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            actions::GameActionsPlugin,
            assets::SetPlugin,
            components::CardComponentsPlugin,
            transition::TransitionsPlugin,
        ))
        .add_systems(PreUpdate, spawn_card.run_if(in_state(AppStates::Playing)));
    }
}
