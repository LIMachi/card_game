use bevy::prelude::*;

pub mod card_click_handler;
pub mod pass_turn_and_attack_buttons;

use crate::utils::ray_caster::update_ray_cast;
use card_click_handler::{card_click_handler, card_hover};
use pass_turn_and_attack_buttons::{attack_button, pass_turn_button};

pub struct PlayerInteractionPlugin;

impl Plugin for PlayerInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, card_hover.after(update_ray_cast))
            .add_systems(
                Update,
                (card_click_handler, pass_turn_button, attack_button),
            );
    }
}
