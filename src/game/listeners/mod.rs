use crate::cards::actions::CardActions;
use crate::prelude::*;

pub fn reset_actions_when_leaving_play(
    mut cards: Query<&mut CardActions, Or<(Added<DiscardPile>, Added<Scrapyard>)>>,
) {
    for mut actions in cards.iter_mut() {
        actions.reset();
    }
}

pub struct ListenersPlugin;

impl Plugin for ListenersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, reset_actions_when_leaving_play);
    }
}
