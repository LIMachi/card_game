use bevy::prelude::*;

pub mod app;
pub mod turn;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<State<app::AppStates>>()
            .register_type::<NextState<app::AppStates>>()
            .add_state::<app::AppStates>();
    }
}
