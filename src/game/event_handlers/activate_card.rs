use crate::game::events::{CardActions, GameEvent, GameEvents};
use crate::game::routines::RoutineManager;
use crate::players::Player;
use crate::prelude::*;
use crate::states::turn::TurnStates;

pub fn activate_card<const PLAYER: u8>(
    mut events: ResMut<GameEvent>,
    mut routines: ResMut<RoutineManager>,
    turn: Res<State<TurnStates>>,
    used_cards: Query<(Entity, &CardIndex), (With<Player<PLAYER>>, With<UsedCards>)>,
    bases: Query<(Entity, &CardIndex), (With<Player<PLAYER>>, With<Bases>)>,
    blobs: Query<
        Entity,
        (
            With<Player<PLAYER>>,
            Or<(With<UsedCards>, With<Bases>)>,
            With<Blob>,
        ),
    >,
    machine_cults: Query<
        Entity,
        (
            With<Player<PLAYER>>,
            Or<(With<UsedCards>, With<Bases>)>,
            With<MachineCult>,
        ),
    >,
    trade_federations: Query<
        Entity,
        (
            With<Player<PLAYER>>,
            Or<(With<UsedCards>, With<Bases>)>,
            With<TradeFederation>,
        ),
    >,
    star_empires: Query<
        Entity,
        (
            With<Player<PLAYER>>,
            Or<(With<UsedCards>, With<Bases>)>,
            With<StarEmpire>,
        ),
    >,
    mut on_play: Query<&mut OnPlay>,
    mut on_scrap: Query<&mut OnScrap>,
    mut combo_blob: Query<&mut ComboBlob>,
    mut combo_machine_cult: Query<&mut ComboMachineCult>,
    mut combo_trade_federation: Query<&mut ComboTradeFederation>,
    mut combo_star_empire: Query<&mut ComboStarEmpire>,
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
                match action {
                    CardActions::Primary => {
                        if let Ok(mut on_play) = on_play.get_mut(card) {
                            if !on_play.1 {
                                routines.activate_card(card, on_play.0.clone());
                                ok = true;
                                on_play.1 = true;
                            }
                        }
                    }
                    CardActions::Scrap => {
                        if let Ok(mut on_scrap) = on_scrap.get_mut(card) {
                            if !on_scrap.1 {
                                routines.activate_card(card, on_scrap.0.clone());
                                ok = true;
                                on_scrap.1 = true;
                            }
                        }
                    }
                    CardActions::Ally => {
                        if let Ok(mut blob) = combo_blob.get_mut(card) {
                            if !blob.1 {
                                if blobs.iter().count() >= 2 {
                                    routines.activate_card(card, blob.0.clone());
                                    ok = true;
                                    blob.1 = true;
                                }
                            }
                        }
                        if let Ok(mut machine_cult) = combo_machine_cult.get_mut(card) {
                            if !machine_cult.1 {
                                if machine_cults.iter().count() >= 2 {
                                    routines.activate_card(card, machine_cult.0.clone());
                                    ok = true;
                                    machine_cult.1 = true;
                                }
                            }
                        }
                        if let Ok(mut trade_federation) = combo_trade_federation.get_mut(card) {
                            if !trade_federation.1 {
                                if trade_federations.iter().count() >= 2 {
                                    routines.activate_card(card, trade_federation.0.clone());
                                    ok = true;
                                    trade_federation.1 = true;
                                }
                            }
                        }
                        if let Ok(mut star_empire) = combo_star_empire.get_mut(card) {
                            if !star_empire.1 {
                                if star_empires.iter().count() >= 2 {
                                    routines.activate_card(card, star_empire.0.clone());
                                    ok = true;
                                    star_empire.1 = true;
                                }
                            }
                        }
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
