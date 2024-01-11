use bevy::prelude::*;

pub mod buttons;
pub mod card_click_handler;

use crate::game::player_interaction::buttons::selection_validation_button;
use crate::game::player_interaction::card_click_handler::{
    focus_card_handler, selection_click, selection_feedback,
};
use crate::game::GameStates;
use crate::utils::ray_caster::update_ray_cast;
use buttons::{attack_button, pass_turn_button};
use card_click_handler::{card_click_handler, card_hover};

pub struct PlayerInteractionPlugin;

impl Plugin for PlayerInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            card_hover
                .after(update_ray_cast)
                .run_if(in_state(GameStates::MainLoop)),
        )
        .add_systems(
            Update,
            (
                focus_card_handler,
                (card_click_handler, pass_turn_button, attack_button)
                    .run_if(in_state(GameStates::MainLoop)),
                (
                    selection_feedback,
                    selection_click,
                    selection_validation_button,
                )
                    .run_if(in_state(GameStates::SelectionInput)),
            ),
        );
    }
}
