use crate::game::events::{GameEvent, GameEvents};
use crate::players::MAXIMUM_PLAYERS;
use crate::prelude::*;
use crate::states::turn::TurnStates;
use crate::ui::player_counters::{AttackButton, PassTurnButton};

pub fn pass_turn_button(
    mouse_buttons: Res<Input<MouseButton>>,
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
    mouse_buttons: Res<Input<MouseButton>>,
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
