use crate::cards::actions::CardActions;
use crate::game::events::{GameEvent, GameEvents};
use crate::game::routines::RoutineManager;
use crate::players::Player;
use crate::prelude::*;

pub fn play_card<const PLAYER: u8>(
    mut event: ResMut<GameEvent>,
    mut routines: ResMut<RoutineManager>,
    mut hand: Query<
        (Entity, &CardIndex, Option<&Ship>, &mut CardActions),
        (With<Hand>, With<Player<PLAYER>>),
    >,
) {
    if let Some(&GameEvents::PlayCard(slot)) = event.get_unprocessed() {
        let slot = slot as usize;
        for (e, i, s, mut a) in hand.iter_mut() {
            if i.0 == slot {
                routines.play(PLAYER, e, slot, s.is_none());
                if s.is_some() && a.play_action_available() {
                    routines.activate_card(e, a.use_play_action().unwrap().clone());
                }
            }
        }
        event.set_processed();
    }
}
