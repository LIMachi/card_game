mod discard;
mod draw;
mod reload_market;
mod shuffle;

use crate::prelude::*;
use std::collections::VecDeque;

#[derive(Reflect, Debug, Default)]
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
    Discard {
        player: u8,
        card: Entity,
        running: bool,
    },
    ReloadMarket {
        slot: u8,
        card: Option<Entity>,
        scrapyard_to_deck: bool,
    },
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct RoutineManager(pub VecDeque<Routines>);

impl RoutineManager {
    pub fn routine(&mut self) -> Option<&mut Routines> {
        self.0.front_mut()
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
        self.0.push_back(Routines::Discard {
            player,
            card,
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
                    discard::discard::<0>,
                    discard::discard::<1>,
                ),
            );
    }
}
