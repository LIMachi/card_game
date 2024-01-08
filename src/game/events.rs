use crate::game::event_handlers::event_handler_dispatcher;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Serialize, Deserialize, Copy, Clone, Debug)]
pub enum CardActions {
    Primary,
    Ally,
    Scrap,
    Indexed(u8),
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
        action: u8,
    },
    BuyCard(BuyFrom),
    Attack {
        player: u8,
        as_much_as_possible: bool,
        base_index: Option<u32>,
    },
    PassTurn,
    #[default]
    Debug, //special event that is here to freeze the game (since it can't be cleared by the main event manager)
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct GameEvent {
    processed: bool, //if true, no event will be fired while the head stays there
    flags: usize,    //general purpose flags available to game events if they need local data
    head: usize,     //log access offset by 1
    log: Vec<GameEvents>,
}

impl GameEvent {
    pub fn flags(&mut self) -> &mut usize {
        &mut self.flags
    }

    pub fn get_unprocessed(&self) -> Option<&GameEvents> {
        if !self.processed && self.head > 0 {
            self.log.get(self.head - 1)
        } else {
            None
        }
    }

    pub fn push(&mut self, event: GameEvents) -> &mut Self {
        if self.head == 0 || (self.processed && self.head == self.log.len()) {
            self.processed = false;
            self.head += 1;
        }
        self.log.push(event);
        self
    }

    pub fn set_processed(&mut self) -> Option<&GameEvents> {
        if !self.processed {
            self.flags = 0;
            if self.head == self.log.len() {
                self.processed = true;
            } else {
                self.head += 1;
            }
        }
        self.get_unprocessed()
    }

    pub fn cancel(&mut self) -> Option<GameEvents> {
        if self.head == self.log.len() {
            self.flags = 0;
            self.processed = true;
        }
        if self.head > 0 {
            self.head -= 1;
            Some(self.log.remove(self.head))
        } else {
            None
        }
    }
}

pub struct GameEventsPlugin;

impl Plugin for GameEventsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameEvent>()
            .init_resource::<GameEvent>()
            .add_systems(Update, event_handler_dispatcher);
    }
}
