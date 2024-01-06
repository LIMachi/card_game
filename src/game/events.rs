use crate::game::event_handlers::event_handler_dispatcher;
use crate::game::routines::RoutinesPlugin;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Serialize, Deserialize, Copy, Clone, Debug)]
pub enum CardActions {
    Primary,
    Ally,
    Scrap,
}

#[derive(Reflect, Serialize, Deserialize, Copy, Clone, Debug)]
pub enum BuyFrom {
    Market(u8),
    Joker,
}

//a full game should be able to be played using only those actions
//it should be possible to check the state of the board at any point if we use setup/pass turn as key frames
//(since we only need the state of the player hand, bases, discard pile and decks)
//(and the market, of course)
//this seems easy enough to send over the network

#[derive(Reflect, Serialize, Deserialize, Copy, Clone, Debug, Default)]
pub enum GameEvents {
    Setup {
        seed: u64,
        set: u8,
        players: u8,
        starting_player: u8,
    },
    PlayCard(u8),
    ActivateCard {
        base: bool,
        index: u32,
        action: CardActions,
    },
    BuyCard(BuyFrom),
    Attack {
        player: u8,
        base_index: Option<u32>,
    },
    PassTurn,
    #[default]
    Debug, //special event that is here to freeze the game (since it can't be cleared by the main event manager)
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct GameEvent {
    processed: bool,
    flags: usize,
    head: usize,
    log: Vec<GameEvents>,
}

impl GameEvent {
    pub fn flags(&mut self) -> &mut usize {
        &mut self.flags
    }

    pub fn get_unprocessed(&self) -> Option<&GameEvents> {
        if !self.processed {
            self.log.get(self.head)
        } else {
            None
        }
    }

    pub fn push(&mut self, event: GameEvents) -> &mut Self {
        if self.processed && self.head == self.log.len() - 1 {
            self.processed = false;
            self.head += 1;
        }
        self.log.push(event);
        self
    }

    pub fn set_processed(&mut self) -> Option<&GameEvents> {
        if !self.processed {
            if self.head == self.log.len() {
                self.processed = true;
            } else {
                self.head += 1;
            }
        }
        self.get_unprocessed()
    }
}

pub struct GameEventsPlugin;

impl Plugin for GameEventsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameEvent>()
            .init_resource::<GameEvent>()
            .add_plugins(RoutinesPlugin)
            // .add_systems(Startup, dispatcher_setup)
            .add_systems(Update, event_handler_dispatcher);
    }
}
