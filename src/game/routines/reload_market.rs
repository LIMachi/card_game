use crate::game::routines::{RoutineManager, Routines};
use crate::prelude::*;

pub fn reload_market(
    mut commands: Commands,
    mut manager: ResMut<RoutineManager>,
    market: Query<Entity, With<MarketRow>>,
    deck: Query<(Entity, &CardIndex), With<MarketDeck>>,
    scrapyard: Query<Entity, With<Scrapyard>>,
) {
    let mut finished = false;
    let mut send_shuffle = false;
    if let Some(Routines::ReloadMarket {
        slot,
        card,
        scrapyard_to_deck,
    }) = manager.routine_mut()
    {
        if let Some(card) = card {
            if market.contains(*card) {
                finished = true;
            }
        } else if *scrapyard_to_deck {
            if deck.is_empty() {
                return;
            }
            send_shuffle = true;
            *scrapyard_to_deck = false;
        } else {
            if deck.is_empty() {
                //FIXME: check for empty market rule
                if !scrapyard.is_empty() {
                    for card in scrapyard.iter() {
                        commands.entity(card).insert(StartTransition {
                            owner: CardOwners::Market,
                            stack: Stacks::MarketDeck,
                            index: CardIndex(0), //will be shuffled anyway
                            visibility: CardVisibility::Hidden,
                            length: 0.5,
                        });
                    }
                    *scrapyard_to_deck = true;
                    return;
                } else {
                    finished = true;
                }
            } else {
                for (e, index) in deck.iter() {
                    if index.0 == 0 {
                        *card = Some(e);
                        commands.entity(e).insert(StartTransition {
                            owner: CardOwners::Market,
                            stack: Stacks::MarketRow,
                            index: CardIndex(*slot as usize),
                            visibility: CardVisibility::Visible,
                            length: 0.5,
                        });
                    } else {
                        commands.entity(e).insert(StartTransition {
                            owner: CardOwners::Market,
                            stack: Stacks::MarketDeck,
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
        manager.shuffle(CardOwners::Market, Stacks::MarketDeck, true);
    }
}
