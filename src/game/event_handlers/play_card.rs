use crate::game::events::{GameEvent, GameEvents};
use crate::game::routines::RoutineManager;
use crate::players::Player;
use crate::prelude::*;

pub fn play_card<const PLAYER: u8>(
    mut event: ResMut<GameEvent>,
    mut routines: ResMut<RoutineManager>,
    hand: Query<(Entity, &CardIndex, Option<&Ship>), (With<Hand>, With<Player<PLAYER>>)>,
) {
    if let Some(&GameEvents::PlayCard(slot)) = event.get_unprocessed() {
        let slot = slot as usize;
        for (e, i, s) in hand.iter() {
            if i.0 == slot {
                routines.play(PLAYER, e, slot, s.is_none());
            }
        }
        event.set_processed();
    }
}
