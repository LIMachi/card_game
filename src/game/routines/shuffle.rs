use crate::game::routines::{RoutineManager, Routines};
use crate::players::Player;
use crate::prelude::*;
use crate::prelude::{
    CardIndex, CardOwners, Commands, Entity, PlayerDeck, Query, Stacks, StartTransition, With,
};
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub fn shuffle(
    mut commands: Commands,
    mut manager: ResMut<RoutineManager>,
    player_0_deck: Query<Entity, (With<Player<0>>, With<PlayerDeck>)>,
    player_1_deck: Query<Entity, (With<Player<1>>, With<PlayerDeck>)>,
    market_deck: Query<Entity, With<MarketDeck>>,
) {
    let mut finished = false;
    if let Some(Routines::Shuffle {
        owner,
        stack,
        running,
    }) = manager.routine_mut()
    {
        match (*owner, *stack) {
            (CardOwners::Player(0), Stacks::PlayerDeck) => {
                if *running {
                    if player_0_deck.is_empty() {
                        return;
                    }
                    finished = true;
                } else {
                    let mut v: Vec<Entity> = player_0_deck.iter().collect();
                    v.shuffle(&mut thread_rng());
                    for (i, e) in v.iter().enumerate() {
                        commands.entity(*e).insert(StartTransition {
                            owner: *owner,
                            stack: *stack,
                            index: CardIndex(i),
                            visibility: CardVisibility::Hidden,
                            length: 0.5,
                        });
                    }
                    *running = true;
                }
            }
            (CardOwners::Player(1), Stacks::PlayerDeck) => {
                if *running {
                    if player_1_deck.is_empty() {
                        return;
                    }
                    finished = true;
                } else {
                    let mut v: Vec<Entity> = player_1_deck.iter().collect();
                    v.shuffle(&mut thread_rng());
                    for (i, e) in v.iter().enumerate() {
                        commands.entity(*e).insert(StartTransition {
                            owner: *owner,
                            stack: *stack,
                            index: CardIndex(i),
                            visibility: CardVisibility::Hidden,
                            length: 0.5,
                        });
                    }
                    *running = true;
                }
            }
            (CardOwners::Market, Stacks::MarketDeck) => {
                if *running {
                    if market_deck.is_empty() {
                        return;
                    }
                    finished = true;
                } else {
                    let mut v: Vec<Entity> = market_deck.iter().collect();
                    v.shuffle(&mut thread_rng());
                    for (i, e) in v.iter().enumerate() {
                        commands.entity(*e).insert(StartTransition {
                            owner: *owner,
                            stack: *stack,
                            index: CardIndex(i),
                            visibility: CardVisibility::Hidden,
                            length: 0.5,
                        });
                    }
                    *running = true;
                }
            }
            _ => {}
        }
    }
    if finished {
        manager.finish();
    }
}
