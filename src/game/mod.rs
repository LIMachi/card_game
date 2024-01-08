use crate::game::events::GameEventsPlugin;
use crate::game::listeners::ListenersPlugin;
use crate::game::player_interaction::PlayerInteractionPlugin;
use crate::game::routines::RoutinesPlugin;
use crate::prelude::App;
use bevy::prelude::Plugin;

pub mod event_handlers;
pub mod events;
pub mod listeners;
mod player_interaction;
pub mod routines;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RoutinesPlugin,
            ListenersPlugin,
            GameEventsPlugin,
            PlayerInteractionPlugin,
        ));
    }
}
