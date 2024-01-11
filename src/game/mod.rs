pub mod event_handlers;
pub mod events;
pub mod listeners;
mod player_interaction;
pub mod routines;

use crate::game::routines::move_to_stack::NextBuyOnDeckFlag;
use crate::prelude::*;
use bevy::prelude::Plugin;
use events::GameEventsPlugin;
use listeners::ListenersPlugin;
use player_interaction::PlayerInteractionPlugin;
use routines::RoutinesPlugin;

#[derive(States, Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Reflect)]
#[reflect(Default)]
pub enum GameStates {
    #[default]
    MainLoop, //main game loop, allow focus, card interaction, button interaction
    ChoiceInput, //chose between prompts. game events are buffered, card interaction and button interaction are disabled
    SelectionInput, //select a card from a list. game events are buffered, card interaction is redirected to the selection system, buttons are disabled
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<NextBuyOnDeckFlag>()
            .init_resource::<NextBuyOnDeckFlag>()
            .register_type::<GameStates>()
            .register_type::<NextState<GameStates>>()
            .register_type::<State<GameStates>>()
            .add_state::<GameStates>()
            .add_plugins((
                RoutinesPlugin,
                ListenersPlugin,
                GameEventsPlugin,
                PlayerInteractionPlugin,
            ));
    }
}
