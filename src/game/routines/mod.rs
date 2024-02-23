mod activate_card;
pub mod card_action;
mod draw;
pub mod move_to_stack;
mod reload_market;
mod selection;
mod shuffle;

use crate::cards::actions::{Action, ActionCondition, KindMask};
use crate::game::events::CardActions;
use crate::game::routines::card_action::Selection;
use crate::prelude::*;
use bevy::utils::HashSet;
use std::collections::VecDeque;

#[derive(Reflect, Debug, Default, Clone)]
pub struct SelectionFilter {
    pub stacks: Vec<Stacks>,
    pub owners: Vec<CardOwners>,
    pub kinds: KindMask,
    pub min_cost: u8,
    pub max_cost: u8,
}

#[derive(Reflect, Debug, Default, Clone)]
pub enum Routines {
    #[default]
    Debug,
    Draw {
        player: u8,
        drawn: Option<Entity>,
        discard_to_deck: bool,
    },
    Shuffle {
        owner: CardOwners,
        stack: Stacks,
        running: bool,
    },
    ReloadMarket {
        slot: u8,
        card: Option<Entity>,
        scrapyard_to_deck: bool,
    },
    PushCardToStack {
        card: Entity,
        owner: CardOwners,
        stack: Stacks,
        index: Option<usize>, //None -> search best slot to insert
        visibility: CardVisibility,
        running: bool,
    },
    ActivateCard {
        card: Entity,
        owner: u8,
        index: u8,
        set: ActionSet,
        running: bool,
    },
    CardAction {
        card: Entity,
        owner: u8,
        ability_index: u8,
        action_index: u8,
        action: Action,
    },
    Selection {
        filters: Vec<SelectionFilter>,
        min: usize,
        max: usize,
        running: bool,
    },
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct RoutineManager(pub VecDeque<Routines>);

impl RoutineManager {
    pub fn selection(
        &mut self,
        min: usize,
        max: usize,
        stack: Stacks,
        owner: CardOwners,
        kinds: KindMask,
        min_cost: u8,
        max_cost: u8,
    ) {
        self.0.push_front(Routines::Selection {
            filters: vec![SelectionFilter {
                stacks: vec![stack],
                owners: vec![owner],
                kinds,
                min_cost,
                max_cost,
            }],
            min,
            max,
            running: false,
        });
    }

    pub fn extended_selection(&mut self, min: usize, max: usize, filters: Vec<SelectionFilter>) {
        self.0.push_front(Routines::Selection {
            filters,
            min,
            max,
            running: false,
        });
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn routine_mut(&mut self) -> Option<&mut Routines> {
        self.0.front_mut()
    }

    pub fn routine(&self) -> Option<Routines> {
        self.0.front().cloned()
    }

    pub fn finish(&mut self) {
        self.0.pop_front();
    }

    pub fn draw(&mut self, player: u8, prioritize: bool) {
        if prioritize {
            self.0.push_front(Routines::Draw {
                player,
                drawn: None,
                discard_to_deck: false,
            });
        } else {
            self.0.push_back(Routines::Draw {
                player,
                drawn: None,
                discard_to_deck: false,
            });
        }
    }

    pub fn reload_market(&mut self, slot: u8) {
        self.0.push_back(Routines::ReloadMarket {
            slot,
            card: None,
            scrapyard_to_deck: false,
        });
    }

    pub fn shuffle(&mut self, owner: CardOwners, stack: Stacks, prioritize: bool) {
        if prioritize {
            self.0.push_front(Routines::Shuffle {
                owner,
                stack,
                running: false,
            });
        } else {
            self.0.push_back(Routines::Shuffle {
                owner,
                stack,
                running: false,
            });
        }
    }

    pub fn discard(&mut self, player: u8, card: Entity) {
        self.0.push_back(Routines::PushCardToStack {
            card,
            owner: CardOwners::Player(player),
            stack: Stacks::DiscardPile,
            index: Some(0),
            visibility: CardVisibility::Visible,
            running: false,
        });
    }

    pub fn scrap(&mut self, card: Entity) {
        self.0.push_back(Routines::PushCardToStack {
            card,
            owner: CardOwners::Market,
            stack: Stacks::Scrapyard,
            index: Some(0),
            visibility: CardVisibility::Visible,
            running: false,
        });
    }

    pub fn play(&mut self, player: u8, card: Entity, slot: usize, base: bool) {
        self.0.push_back(Routines::PushCardToStack {
            card,
            owner: CardOwners::Player(player),
            stack: if base {
                Stacks::Bases
            } else {
                Stacks::UsedCards
            },
            index: Some(if base { 0 } else { slot }),
            visibility: CardVisibility::Visible,
            running: false,
        });
    }

    pub fn activate_card(&mut self, owner: u8, card: Entity, index: u8, set: ActionSet) {
        self.0.push_back(Routines::ActivateCard {
            card,
            owner,
            index,
            set,
            running: false,
        });
    }

    pub fn action(
        &mut self,
        owner: u8,
        card: Entity,
        ability_index: u8,
        action_index: u8,
        action: Action,
    ) {
        self.0.push_back(Routines::CardAction {
            card,
            owner,
            ability_index,
            action_index,
            action,
        });
    }
}

pub struct RoutinesPlugin;

impl Plugin for RoutinesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Selection>()
            .init_resource::<Selection>()
            .register_type::<Routines>()
            .register_type::<RoutineManager>()
            .init_resource::<RoutineManager>()
            .add_systems(
                Update,
                (
                    draw::draw_routine::<0>,
                    draw::draw_routine::<1>,
                    shuffle::shuffle,
                    reload_market::reload_market,
                    move_to_stack::move_to_stack,
                    activate_card::activate_card,
                    card_action::card_action,
                    selection::selection,
                ),
            );
    }
}
