use crate::cards::actions::KindMask;
use crate::cards::components::factions::CardFaction;
use crate::game::routines::{RoutineManager, SelectionFilter};
use crate::players::{Player, PlayerTurnTracker};
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Default, Reflect, Copy, Clone, Eq, PartialEq)]
pub enum Uniques {
    #[default]
    Unimplemented,
    BlobWorld,
    BrainWorld,
    EmbassyYacht,
    FleetHQ,
    RecyclingStation,
    StealthNeedle,
}

impl Uniques {
    pub fn card_action(&self, world: &mut World, card: Entity, owner: u8) {
        match self {
            Uniques::BlobWorld => {
                //simple check for the amount of blob cards played and queueing X draws
                let draw = if owner == 0 {
                    world
                        .query_filtered::<&PlayerTurnTracker, With<Player<1>>>()
                        .get_single_mut(world)
                } else {
                    world
                        .query_filtered::<&PlayerTurnTracker, With<Player<0>>>()
                        .get_single_mut(world)
                }
                .unwrap()
                .faction_counters
                .get(&CardFaction::Blob)
                .unwrap()
                .ships_played;
                let mut routines = world.resource_mut::<RoutineManager>();
                routines.finish();
                for _ in 0..draw {
                    routines.draw(owner);
                }
            }
            Uniques::BrainWorld => {
                //equivalent to scrap hand/discard, but we use the size of the return to redraw cards
                let mut selection =
                    world.resource_mut::<crate::game::routines::card_action::Selection>();
                if selection.finished {
                    selection.finished = false;
                    let test: Vec<Entity> = selection.cards.drain(..).collect();
                    let mut routines = world.resource_mut::<RoutineManager>();
                    routines.finish();
                    for &card in test.iter() {
                        routines.scrap(card);
                        routines.draw(owner);
                    }
                } else {
                    world.resource_mut::<RoutineManager>().extended_selection(
                        0,
                        2,
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
            Uniques::EmbassyYacht => {
                //simple check for the amount of bases in play and queuing 2 draws
                let draw = if owner == 0 {
                    world
                        .query_filtered::<&PlayerTurnTracker, With<Player<1>>>()
                        .get_single_mut(world)
                } else {
                    world
                        .query_filtered::<&PlayerTurnTracker, With<Player<0>>>()
                        .get_single_mut(world)
                }
                .unwrap()
                .common
                .bases_in_play
                    >= 2;
                let mut routines = world.resource_mut::<RoutineManager>();
                routines.finish();
                if draw {
                    routines.draw(owner);
                    routines.draw(owner);
                }
            }
            Uniques::FleetHQ => {
                //toggle a flag/resource/player component that detects when a ship is played and increase the attack
                world.resource_mut::<RoutineManager>().finish(); //FIXME: for now, do nothing
            }
            Uniques::RecyclingStation => {
                //equivalent to immediate discard, but we use the size of the return to redraw cards
                let mut selection =
                    world.resource_mut::<crate::game::routines::card_action::Selection>();
                if selection.finished {
                    selection.finished = false;
                    let test: Vec<Entity> = selection.cards.drain(..).collect();
                    let mut routines = world.resource_mut::<RoutineManager>();
                    routines.finish();
                    for &card in test.iter() {
                        routines.discard(owner, card);
                        routines.draw(owner);
                    }
                } else {
                    world.resource_mut::<RoutineManager>().selection(
                        0,
                        2,
                        Stacks::Hand,
                        CardOwners::Player(owner),
                        KindMask::Any,
                        0,
                        255,
                    );
                }
            }
            Uniques::StealthNeedle => {
                //now that one hard card to do...
                world.resource_mut::<RoutineManager>().finish(); //FIXME: for now, do nothing
            }
            _ => {}
        }
    }
}

impl Display for Uniques {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BlobWorld => f.write_str("draw a card for each Blob card that you've played this turn"),
            Self::BrainWorld => f.write_str("Scrap up to 2 cards from you hand and/or discard pile. Draw a card for each card scrapped this way"),
            Self::EmbassyYacht => f.write_str("if you have 2+ bases in play, draw 2 cards"),
            Self::FleetHQ => f.write_str("Whenever you play a ship this turn, gain 1 attack"),
            Self::RecyclingStation => f.write_str("discard up to two cards, then draw that many cards"),
            Self::StealthNeedle => f.write_str("Copy another ship you've played this turn. Stealth Needle has that ship's faction in addition to Machine Cult"),
            _ => f.write_str("UNIMPLEMENTED"),
        }
    }
}
