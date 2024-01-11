use crate::cards::actions::CardActions;
use crate::game::events::{GameEvent, GameEvents};
use crate::game::routines::RoutineManager;
use crate::players::{Player, PlayerTurnTracker};
use crate::prelude::*;
use crate::states::turn::TurnStates;

pub fn activate_card<const PLAYER: u8>(
    mut events: ResMut<GameEvent>,
    mut routines: ResMut<RoutineManager>,
    turn: Res<State<TurnStates>>,
    used_cards: Query<(Entity, &CardIndex), (With<Player<PLAYER>>, With<UsedCards>)>,
    bases: Query<(Entity, &CardIndex), (With<Player<PLAYER>>, With<Bases>)>,
    mut card_actions: Query<&mut CardActions, With<Player<PLAYER>>>,
    player_0_tracker: Query<&PlayerTurnTracker, With<Player<0>>>,
    player_1_tracker: Query<&PlayerTurnTracker, With<Player<1>>>,
) {
    if let Some(&GameEvents::ActivateCard {
        base,
        index,
        action,
    }) = events.get_unprocessed()
    {
        if let TurnStates::PlayerTurn(p) = turn.get() {
            if *p != PLAYER {
                return;
            }
            if let Ok(tracker) = if *p == 0 {
                player_0_tracker.get_single()
            } else {
                player_1_tracker.get_single()
            } {
                let mut card = Entity::PLACEHOLDER;
                if base {
                    for (e, i) in bases.iter() {
                        if i.0 == index as usize {
                            card = e;
                        }
                    }
                } else {
                    for (e, i) in used_cards.iter() {
                        if i.0 == index as usize {
                            card = e;
                        }
                    }
                }
                let mut ok = false;
                if card != Entity::PLACEHOLDER {
                    if let Ok(mut ca) = card_actions.get_mut(card) {
                        if let Some((set, scrap)) = ca.use_action(action, tracker) {
                            routines.activate_card(PLAYER, card, action, set.clone());
                            if scrap {
                                routines.scrap(card);
                            }
                            ok = true;
                        }
                    }
                }
                if ok {
                    events.set_processed();
                } else {
                    events.cancel();
                }
            }
        }
    }
}
