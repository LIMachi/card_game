use crate::game::routines::{RoutineManager, Routines};
use crate::players::Player;
use crate::prelude::*;
use crate::prelude::{CardIndex, Commands, Entity, Query, With};

pub fn discard<const PLAYER: u8>(
    mut commands: Commands,
    mut manager: ResMut<RoutineManager>,
    discard_pile: Query<(Entity, &CardIndex), (With<DiscardPile>, With<Player<PLAYER>>)>,
    all_cards: Query<(Entity, &CardIndex, &CardOwners, &Stacks, &CardVisibility)>,
) {
    let mut finished = false;
    if let Some(Routines::Discard {
        player,
        card,
        running,
    }) = manager.routine()
    {
        if *player != PLAYER {
            return;
        }
        if discard_pile.contains(*card) {
            finished = true;
        } else if !*running {
            if let Ok((card, &index, &owner, &stack, &visibility)) = all_cards.get(*card) {
                //move cards that where below the one being discarded up one index if needed
                if !stack.keep_empty_spaces() {
                    for (icard, iindex, iowner, istack, ivisibility) in all_cards.iter() {
                        if iindex.0 > index.0 && *iowner == owner && *istack == stack {
                            //the card was below the one discarded (higher index, same owner and stack)
                            commands.entity(icard).insert(StartTransition {
                                owner,
                                stack,
                                index: CardIndex(iindex.0 - 1),
                                visibility: *ivisibility,
                                length: 0.5,
                            });
                        }
                    }
                }
                for (icard, iindex) in discard_pile.iter() {
                    commands.entity(icard).insert(StartTransition {
                        owner: CardOwners::Player(PLAYER),
                        stack: Stacks::DiscardPile,
                        index: CardIndex(iindex.0 + 1),
                        visibility: CardVisibility::Visible,
                        length: 0.5,
                    });
                }
                commands.entity(card).insert(StartTransition {
                    owner: CardOwners::Player(PLAYER),
                    stack: Stacks::DiscardPile,
                    index: CardIndex(0),
                    visibility: CardVisibility::Visible,
                    length: 0.5,
                });
                *running = true;
            } else {
                //no card matched the query, it might already be in a transition
                //treat this as a finish?
                finished = true;
            }
        }
    }
    if finished {
        manager.finish();
    }
}
