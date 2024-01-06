mod activate_card;
mod attack;
mod buy_card;
mod pass_turn;
mod play_card;
mod setup;

use self::activate_card::activate_card;
use self::attack::attack;
use self::buy_card::buy_card;
use self::pass_turn::pass_turn;
use self::play_card::play_card;
use self::setup::setup;
use crate::game::events::{GameEvent, GameEvents};
use crate::prelude::*;
use crate::states::turn::TurnStates;
use bevy::ecs::system::RunSystemOnce;

pub fn event_handler_dispatcher(world: &mut World) {
    if let Some(&event) = world
        .get_resource::<GameEvent>()
        .and_then(|ge| ge.get_unprocessed())
    {
        // let &dispatcher = world.get_resource::<GameEventDispatcher>().unwrap();
        let &state = world.get_resource::<State<TurnStates>>().unwrap().get();
        match event {
            GameEvents::Setup { .. } => {
                if state == TurnStates::Setup {
                    world.run_system_once(setup);
                }
            }
            GameEvents::PlayCard(_) => {
                if let TurnStates::PlayerTurn(p) = state {
                    match p {
                        0 => world.run_system_once(play_card::<0>),
                        1 => world.run_system_once(play_card::<1>),
                        _ => {}
                    }
                }
            }
            GameEvents::ActivateCard { .. } => {
                if let TurnStates::PlayerTurn(p) = state {
                    match p {
                        0 => world.run_system_once(activate_card::<0>),
                        1 => world.run_system_once(activate_card::<1>),
                        _ => {}
                    }
                }
            }
            GameEvents::BuyCard(_) => {
                if let TurnStates::PlayerTurn(p) = state {
                    match p {
                        0 => world.run_system_once(buy_card::<0>),
                        1 => world.run_system_once(buy_card::<1>),
                        _ => {}
                    }
                }
            }
            GameEvents::Attack { .. } => {
                if let TurnStates::PlayerTurn(p) = state {
                    match p {
                        0 => world.run_system_once(attack::<0>),
                        1 => world.run_system_once(attack::<1>),
                        _ => {}
                    }
                }
            }
            GameEvents::PassTurn => {
                if let TurnStates::PlayerTurn(p) = state {
                    match p {
                        0 => world.run_system_once(pass_turn::<0>),
                        1 => world.run_system_once(pass_turn::<1>),
                        _ => {}
                    }
                }
            }
            GameEvents::Debug => {}
        }
    }
}
