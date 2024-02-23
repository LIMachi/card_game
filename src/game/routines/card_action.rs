use crate::cards::actions::{Action, KindMask};
use crate::game::routines::move_to_stack::NextBuyOnDeckFlag;
use crate::game::routines::{RoutineManager, Routines, SelectionFilter};
use crate::players::{Player, PlayerAttack, PlayerEconomy, PlayerLife, PlayerTurnTracker};
use crate::prelude::*;
use bevy::utils::HashMap;

//before: insert selectable in entities
//during: monitor clicks on selectable, visual feedback for selected
//finish: collect selected entities in choice handler resource, remove selected/selectable components

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct Selection {
    pub cards: Vec<Entity>,
    pub finished: bool,
    pub min: usize,
    pub max: usize,
}

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Selected;

pub fn card_action(world: &mut World) {
    if let Some(Routines::CardAction {
        card,
        owner,
        ability_index,
        action_index,
        action,
    }) = world.resource::<RoutineManager>().routine()
    {
        match action {
            Action::Eco(qty) => {
                if owner == 0 {
                    if let Ok(mut eco) = world
                        .query_filtered::<&mut PlayerEconomy, With<Player<0>>>()
                        .get_single_mut(world)
                    {
                        eco.0 += qty as i32;
                    }
                }
                if owner == 1 {
                    if let Ok(mut eco) = world
                        .query_filtered::<&mut PlayerEconomy, With<Player<1>>>()
                        .get_single_mut(world)
                    {
                        eco.0 += qty as i32;
                    }
                }
                world.resource_mut::<RoutineManager>().finish();
            }
            Action::Atk(qty) => {
                if owner == 0 {
                    if let Ok(mut atk) = world
                        .query_filtered::<&mut PlayerAttack, With<Player<0>>>()
                        .get_single_mut(world)
                    {
                        atk.0 += qty as i32;
                    }
                }
                if owner == 1 {
                    if let Ok(mut atk) = world
                        .query_filtered::<&mut PlayerAttack, With<Player<1>>>()
                        .get_single_mut(world)
                    {
                        atk.0 += qty as i32;
                    }
                }
                world.resource_mut::<RoutineManager>().finish();
            }
            Action::Life(qty) => {
                if owner == 0 {
                    if let Ok(mut life) = world
                        .query_filtered::<&mut PlayerLife, With<Player<0>>>()
                        .get_single_mut(world)
                    {
                        life.0 += qty as i32;
                    }
                }
                if owner == 1 {
                    if let Ok(mut life) = world
                        .query_filtered::<&mut PlayerLife, With<Player<1>>>()
                        .get_single_mut(world)
                    {
                        life.0 += qty as i32;
                    }
                }
                world.resource_mut::<RoutineManager>().finish();
            }
            Action::Draw(qty) => {
                let mut routines = world.resource_mut::<RoutineManager>();
                routines.finish();
                for _ in 0..qty {
                    routines.draw(owner, true);
                }
            }
            Action::Discard => {
                let mut selection = world.resource_mut::<Selection>();
                if selection.finished {
                    selection.finished = false;
                    let test: Vec<Entity> = selection.cards.drain(..).collect();
                    let mut routines = world.resource_mut::<RoutineManager>();
                    routines.finish();
                    for &card in test.iter() {
                        routines.discard(owner, card);
                    }
                } else {
                    world.resource_mut::<RoutineManager>().selection(
                        1,
                        1,
                        Stacks::Hand,
                        CardOwners::Player(owner),
                        KindMask::Any,
                        0,
                        255,
                    );
                }
            }
            Action::ScrapHand => {
                let mut selection = world.resource_mut::<Selection>();
                if selection.finished {
                    selection.finished = false;
                    let test: Vec<Entity> = selection.cards.drain(..).collect();
                    let mut routines = world.resource_mut::<RoutineManager>();
                    routines.finish();
                    for &card in test.iter() {
                        routines.scrap(card);
                    }
                } else {
                    world.resource_mut::<RoutineManager>().selection(
                        1,
                        1,
                        Stacks::Hand,
                        CardOwners::Player(owner),
                        KindMask::Any,
                        0,
                        255,
                    );
                }
            }
            Action::ScrapDiscard => {
                let mut selection = world.resource_mut::<Selection>();
                if selection.finished {
                    selection.finished = false;
                    let test: Vec<Entity> = selection.cards.drain(..).collect();
                    let mut routines = world.resource_mut::<RoutineManager>();
                    routines.finish();
                    for &card in test.iter() {
                        routines.scrap(card);
                    }
                } else {
                    world.resource_mut::<RoutineManager>().selection(
                        1,
                        1,
                        Stacks::DiscardPile,
                        CardOwners::Player(owner),
                        KindMask::Any,
                        0,
                        255,
                    );
                }
            }
            Action::ScrapHandOrDiscard => {
                let mut selection = world.resource_mut::<Selection>();
                if selection.finished {
                    selection.finished = false;
                    let test: Vec<Entity> = selection.cards.drain(..).collect();
                    let mut routines = world.resource_mut::<RoutineManager>();
                    routines.finish();
                    for &card in test.iter() {
                        routines.scrap(card);
                    }
                } else {
                    world.resource_mut::<RoutineManager>().extended_selection(
                        1,
                        1,
                        vec![SelectionFilter {
                            stacks: vec![Stacks::Hand, Stacks::DiscardPile],
                            owners: vec![CardOwners::Player(owner)],
                            kinds: KindMask::Any,
                            min_cost: 0,
                            max_cost: 255,
                        }],
                    );
                }
            }
            Action::ScrapMarket => {
                let mut selection = world.resource_mut::<Selection>();
                if selection.finished {
                    selection.finished = false;
                    let tmp: Vec<Entity> = selection.cards.drain(..).collect();
                    let mut map = HashMap::with_capacity(tmp.len());
                    for c in tmp.iter() {
                        map.insert(*c, world.get::<CardIndex>(*c).unwrap().0 as u8);
                    }
                    let mut routines = world.resource_mut::<RoutineManager>();
                    routines.finish();
                    for (card, index) in map.iter() {
                        routines.scrap(*card);
                        routines.reload_market(*index);
                    }
                } else {
                    world.resource_mut::<RoutineManager>().selection(
                        1,
                        1,
                        Stacks::MarketRow,
                        CardOwners::Market,
                        KindMask::Any,
                        0,
                        255,
                    );
                }
            }
            Action::ScrapSelf => {
                let mut routines = world.resource_mut::<RoutineManager>();
                routines.finish();
                routines.discard(owner, card);
            }
            Action::DestroyBase => {
                let mut selection = world.resource_mut::<Selection>();
                if selection.finished {
                    selection.finished = false;
                    let test: Vec<Entity> = selection.cards.drain(..).collect();
                    let mut routines = world.resource_mut::<RoutineManager>();
                    routines.finish();
                    for &card in test.iter() {
                        routines.discard(if owner == 0 { 1 } else { 0 }, card);
                    }
                } else {
                    //complex filter ahead
                    if owner == 0 {
                        if world.query_filtered::<Entity, (With<Bases>, With<Outpost>, With<Player<1>>)>().iter(world).count() != 0 {
                            //first, test for outposts
                            world.resource_mut::<RoutineManager>().selection(
                                1,
                                1,
                                Stacks::Bases,
                                CardOwners::Player(1),
                                KindMask::Outposts,
                                0,
                                255,
                            );
                        } else if world.query_filtered::<Entity, (With<Bases>, With<Outpost>, With<Player<1>>)>().iter(world).count() != 0 {
                            //then test for bases
                            world.resource_mut::<RoutineManager>().selection(
                                1,
                                1,
                                Stacks::Bases,
                                CardOwners::Player(1),
                                KindMask::Bases,
                                0,
                                255,
                            );
                        } else {
                            //then if no base, early return
                            world.resource_mut::<RoutineManager>().finish();
                        }
                    } else {
                        //and do this twice, once for each player
                        if world.query_filtered::<Entity, (With<Bases>, With<Outpost>, With<Player<0>>)>().iter(world).count() != 0 {
                            //first, test for outposts
                            world.resource_mut::<RoutineManager>().selection(
                                1,
                                1,
                                Stacks::Bases,
                                CardOwners::Player(0),
                                KindMask::Outposts,
                                0,
                                255,
                            );
                        } else if world.query_filtered::<Entity, (With<Bases>, With<Outpost>, With<Player<0>>)>().iter(world).count() != 0 {
                            //then test for bases
                            world.resource_mut::<RoutineManager>().selection(
                                1,
                                1,
                                Stacks::Bases,
                                CardOwners::Player(0),
                                KindMask::Bases,
                                0,
                                255,
                            );
                        } else {
                            //then if no base, early return
                            world.resource_mut::<RoutineManager>().finish();
                        }
                    }
                }
            }
            Action::EnemyDiscard => {
                world.resource_mut::<RoutineManager>().finish();
                if owner == 0 {
                    world
                        .query_filtered::<&mut PlayerTurnTracker, With<Player<1>>>()
                        .get_single_mut(world)
                        .unwrap()
                        .cards_to_discard += 1;
                } else {
                    world
                        .query_filtered::<&mut PlayerTurnTracker, With<Player<0>>>()
                        .get_single_mut(world)
                        .unwrap()
                        .cards_to_discard += 1;
                }
                //spawn a prompt to select an enemy player?
                //discards are stored and only applied at the start of that players turn
            }
            Action::FreeBuy {
                min_cost,
                max_cost,
                valid_kinds,
            } => {
                let mut selection = world.resource_mut::<Selection>();
                if selection.finished {
                    selection.finished = false;
                    let tmp: Vec<Entity> = selection.cards.drain(..).collect();
                    let mut map = HashMap::with_capacity(tmp.len());
                    for c in tmp.iter() {
                        map.insert(*c, world.get::<CardIndex>(*c).unwrap().0 as u8);
                    }
                    let mut routines = world.resource_mut::<RoutineManager>();
                    routines.finish();
                    for (card, index) in map.iter() {
                        routines.discard(owner, *card);
                        routines.reload_market(*index);
                    }
                } else {
                    world.resource_mut::<RoutineManager>().selection(
                        1,
                        1,
                        Stacks::MarketRow,
                        CardOwners::Market,
                        valid_kinds,
                        min_cost,
                        max_cost,
                    );
                }
            }
            Action::NextBuyOnDeck(kinds) => {
                world.resource_mut::<RoutineManager>().finish();
                world.resource_mut::<NextBuyOnDeckFlag>().0 = kinds;
            }
            Action::Unique(unique) => {
                unique.card_action(world, card, owner);
            }
        }
    }
}
