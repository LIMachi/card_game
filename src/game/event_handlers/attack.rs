use crate::cards::components::kinds::BaseLife;
use crate::game::events::{GameEvent, GameEvents};
use crate::game::routines::RoutineManager;
use crate::players::{Player, PlayerAttack, PlayerLife};
use crate::prelude::*;
use crate::states::turn::TurnStates;

pub fn attack<const BY: u8, const PLAYER: u8>(
    mut events: ResMut<GameEvent>,
    turn: Res<State<TurnStates>>,
    indexes: Query<(Entity, &CardIndex, &CardKinds), (With<Player<PLAYER>>, With<Bases>)>,
    mut bases: Query<
        &mut BaseLife,
        (
            With<Player<PLAYER>>,
            With<Bases>,
            With<Base>,
            Without<Outpost>,
        ),
    >,
    mut outposts: Query<
        &mut BaseLife,
        (
            With<Player<PLAYER>>,
            With<Bases>,
            With<Outpost>,
            Without<Base>,
        ),
    >,
    mut life: Query<&mut PlayerLife, With<Player<PLAYER>>>,
    mut attack: Query<&mut PlayerAttack, With<Player<BY>>>,
    mut routines: ResMut<RoutineManager>,
) {
    if let Some(&GameEvents::Attack {
        as_much_as_possible,
        player,
        base_index,
    }) = events.get_unprocessed()
    {
        let mut ok = false;

        if let TurnStates::PlayerTurn(p) = turn.get() {
            if let Ok(mut attack) = attack.get_single_mut() {
                if attack.0 > 0 {
                    if let Some(index) = base_index {
                        //attack a base
                        let mut card = Entity::PLACEHOLDER;
                        let mut outpost = false;
                        for (e, i, k) in indexes.iter() {
                            if i.0 == index as usize {
                                match k {
                                    CardKinds::Ship => {
                                        break;
                                    }
                                    CardKinds::Base(_) => {
                                        card = e;
                                        break;
                                    }
                                    CardKinds::Outpost(_) => {
                                        card = e;
                                        outpost = true;
                                        break;
                                    }
                                }
                            }
                        }
                        if card != Entity::PLACEHOLDER {
                            if outpost {
                                let mut life = outposts.get_mut(card).unwrap();
                                let damage = if as_much_as_possible {
                                    life.0.min(attack.0)
                                } else {
                                    1
                                };
                                life.0 -= damage;
                                attack.0 -= damage;
                                if life.0 == 0 {
                                    routines.discard(PLAYER, card);
                                }
                                ok = true;
                            } else {
                                if outposts.is_empty() {
                                    let mut life = bases.get_mut(card).unwrap();
                                    let damage = if as_much_as_possible {
                                        life.0.min(attack.0)
                                    } else {
                                        1
                                    };
                                    life.0 -= damage;
                                    attack.0 -= damage;
                                    if life.0 == 0 {
                                        routines.discard(PLAYER, card);
                                    }
                                    ok = true;
                                } else {
                                    //cannot, should make a visual feedback on outposts
                                }
                            }
                        }
                    } else {
                        //attack the player itself
                        if outposts.is_empty() {
                            let mut life = life.get_single_mut().unwrap();
                            let damage = if as_much_as_possible {
                                life.0.min(attack.0)
                            } else {
                                1
                            };
                            life.0 -= damage;
                            attack.0 -= damage;
                            ok = true;
                        } else {
                            //cannot, should make a visual feedback on outposts
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
