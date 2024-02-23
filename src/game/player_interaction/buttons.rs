use crate::game::events::{GameEvent, GameEvents};
use crate::game::routines::card_action::{Selected, Selection};
use crate::players::MAXIMUM_PLAYERS;
use crate::prelude::*;
use crate::states::turn::TurnStates;
use crate::ui::player_counters::{AttackButton, PassTurnButton};

pub fn pass_turn_button(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    button: Query<&PassTurnButton>,
    mut events: ResMut<GameEvent>,
) {
    if mouse_buttons.just_released(MouseButton::Left)
        || mouse_buttons.just_released(MouseButton::Right)
    {
        if button.get_single().unwrap().hovered {
            events.push(GameEvents::PassTurn);
        }
    }
}

pub fn attack_button(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    button: Query<&AttackButton>,
    mut events: ResMut<GameEvent>,
    turn: Res<State<TurnStates>>,
) {
    if let &TurnStates::PlayerTurn(player) = turn.get() {
        if mouse_buttons.just_released(MouseButton::Left) {
            if button.get_single().unwrap().hovered {
                events.push(GameEvents::Attack {
                    player: (player + 1) % MAXIMUM_PLAYERS as u8,
                    as_much_as_possible: false,
                    base_index: None,
                });
            }
        } else if mouse_buttons.just_released(MouseButton::Right) {
            if button.get_single().unwrap().hovered {
                events.push(GameEvents::Attack {
                    player: (player + 1) % MAXIMUM_PLAYERS as u8,
                    as_much_as_possible: true,
                    base_index: None,
                });
            }
        }
    }
}

pub fn selection_validation_button(
    mut button: Query<(&Interaction, &mut BackgroundColor, &mut BorderColor), Changed<Interaction>>,
    mut selection: ResMut<Selection>,
    selected: Query<Entity, With<Selected>>,
) {
    if !selection.finished {
        let len = selected.iter().count();
        if let Ok((interaction, mut background, mut border)) = button.get_single_mut() {
            match interaction {
                Interaction::Pressed => {
                    if len >= selection.min && len <= selection.max {
                        selection.finished = true;
                    }
                }
                Interaction::Hovered => {
                    if len >= selection.min && len <= selection.max {
                        *background = BackgroundColor(Color::GREEN.with_a(0.9));
                    } else {
                        *background = BackgroundColor(Color::RED.with_a(0.9));
                    }
                }
                Interaction::None => {
                    *background = BackgroundColor(Color::BLACK.with_a(0.9));
                    // *border = Default::default();
                }
            }
        }
    }
}
