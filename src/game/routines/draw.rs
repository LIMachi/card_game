use crate::game::routines::{RoutineManager, Routines};
use crate::players::{LocalPlayer, Player};
use crate::prelude::*;
use crate::prelude::{
    CardIndex, CardOwners, Commands, DiscardPile, Entity, Hand, PlayerDeck, Query, Stacks,
    StartTransition, With, Without,
};

pub fn draw_routine<const PLAYER: u8>(
    mut commands: Commands,
    mut manager: ResMut<RoutineManager>,
    hand: Query<&CardIndex, (With<Hand>, With<Player<PLAYER>>, Without<PlayerDeck>)>,
    discard_pile: Query<Entity, (With<DiscardPile>, With<Player<PLAYER>>, Without<PlayerDeck>)>,
    deck: Query<(Entity, &CardIndex), (With<PlayerDeck>, With<Player<PLAYER>>)>,
    local_player: Res<LocalPlayer>,
) {
    let mut finished = false;
    let mut send_shuffle = false;
    if let Some(Routines::Draw {
        player,
        drawn,
        discard_to_deck,
    }) = manager.routine()
    {
        if *player != PLAYER {
            return;
        }
        if let Some(drawn) = drawn {
            if hand.contains(*drawn) {
                finished = true;
            }
        } else if *discard_to_deck {
            if deck.is_empty() {
                return;
            }
            send_shuffle = true;
            *discard_to_deck = false;
        } else {
            if deck.is_empty() {
                if !discard_pile.is_empty() {
                    for card in discard_pile.iter() {
                        commands.entity(card).insert(StartTransition {
                            owner: CardOwners::Player(PLAYER),
                            stack: Stacks::PlayerDeck,
                            index: CardIndex(0), //will be shuffled anyway
                            visibility: CardVisibility::Hidden,
                            length: 0.5,
                        });
                    }
                    *discard_to_deck = true;
                    return;
                }
            } else {
                //search empty slot (mask trick to allow random iteration)
                let empty_slot = hand
                    .iter()
                    .fold(0u64, |m, v| m | 1u64 << v.0)
                    .trailing_ones();
                for (card, index) in deck.iter() {
                    if index.0 == 0 {
                        *drawn = Some(card);
                        commands.entity(card).insert(StartTransition {
                            owner: CardOwners::Player(PLAYER),
                            stack: Stacks::Hand,
                            index: CardIndex(empty_slot as usize),
                            visibility: if local_player.0 == PLAYER {
                                CardVisibility::Visible
                            } else {
                                CardVisibility::Hidden
                            },
                            length: 0.5,
                        });
                    } else {
                        commands.entity(card).insert(StartTransition {
                            owner: CardOwners::Player(PLAYER),
                            stack: Stacks::PlayerDeck,
                            index: CardIndex(index.0 - 1),
                            visibility: CardVisibility::Hidden,
                            length: 0.5,
                        });
                    }
                }
            }
        }
    }
    if finished {
        manager.finish();
    } else if send_shuffle {
        manager.shuffle(CardOwners::Player(PLAYER), Stacks::PlayerDeck, true);
    }
}
