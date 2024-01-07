use crate::game::events::{BuyFrom, GameEvent, GameEvents};
use crate::game::routines::RoutineManager;
use crate::players::{Player, PlayerEconomy};
use crate::prelude::*;
use crate::states::turn::TurnStates;

pub fn buy_card<const PLAYER: u8>(
    mut event: ResMut<GameEvent>,
    mut routines: ResMut<RoutineManager>,
    turn: Res<State<TurnStates>>,
    mut eco: Query<&mut PlayerEconomy, With<Player<PLAYER>>>,
    jokers: Query<(Entity, &CardIndex, &CardCost), With<JokerDeck>>,
    market: Query<(Entity, &CardIndex, &CardCost), With<MarketRow>>,
) {
    if let Some(&GameEvents::BuyCard(from)) = event.get_unprocessed() {
        if let TurnStates::PlayerTurn(p) = turn.get() {
            if *p != PLAYER {
                return;
            }
            let mut ok = false;
            if let Ok(mut eco) = eco.get_single_mut() {
                let (market, card, cost) = match from {
                    BuyFrom::Market(slot) => {
                        let mut card = None;
                        let mut cost = 2;
                        for (e, i, c) in market.iter() {
                            if i.0 == slot as usize {
                                card = Some(e);
                                cost = c.0;
                            }
                        }
                        (Some(slot), card, cost)
                    }
                    BuyFrom::Joker => {
                        let mut card = None;
                        let mut cost = 2;
                        for (e, i, c) in jokers.iter() {
                            if i.0 == 0usize {
                                card = Some(e);
                                cost = c.0;
                            }
                        }
                        (None, card, cost)
                    }
                };
                if let Some(card) = card {
                    if cost <= eco.0 {
                        eco.0 -= cost;
                        ok = true;
                        routines.discard(PLAYER, card);
                        if let Some(slot) = market {
                            routines.reload_market(slot);
                        }
                    }
                }
            }
            if ok {
                event.set_processed();
            } else {
                event.cancel();
            }
        }
    }
}
