mod activate_card;
mod choice;
mod draw;
mod move_to_stack;
mod reload_market;
mod shuffle;

use crate::game::events::CardActions;
use crate::prelude::*;
use std::collections::VecDeque;

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
        set: ActionSet,
        running: bool,
    },
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct RoutineManager(pub VecDeque<Routines>);

impl RoutineManager {
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

    pub fn draw(&mut self, player: u8) {
        self.0.push_back(Routines::Draw {
            player,
            drawn: None,
            discard_to_deck: false,
        });
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

    pub fn activate_card(&mut self, card: Entity, set: ActionSet) {
        self.0.push_back(Routines::ActivateCard {
            card,
            set,
            running: false,
        });
    }
}

pub struct RoutinesPlugin;

impl Plugin for RoutinesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Routines>()
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
                ),
            );
    }
}
