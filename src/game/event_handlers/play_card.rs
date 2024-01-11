use crate::cards::actions::CardActions;
use crate::game::events::{GameEvent, GameEvents};
use crate::game::routines::RoutineManager;
use crate::players::{Player, PlayerTurnTracker};
use crate::prelude::*;

pub fn play_card<const PLAYER: u8>(
    mut event: ResMut<GameEvent>,
    mut routines: ResMut<RoutineManager>,
    mut hand: Query<
        (Entity, &CardIndex, Option<&Ship>, &mut CardActions),
        (With<Hand>, With<Player<PLAYER>>),
    >,
    trackers: Query<&PlayerTurnTracker, With<Player<PLAYER>>>,
) {
    if let Some(&GameEvents::PlayCard(slot)) = event.get_unprocessed() {
        let slot = slot as usize;
        for (e, i, s, mut a) in hand.iter_mut() {
            if i.0 == slot {
                routines.play(PLAYER, e, 0, s.is_none());
                let trackers = trackers.get_single().unwrap();
                if s.is_some() && a.is_action_available(0, trackers) {
                    routines.activate_card(
                        PLAYER,
                        e,
                        0,
                        a.use_action(0, trackers)
                            .map_or(ActionSet::None, |o| o.0.clone()),
                    );
                }
            }
        }
        event.set_processed();
    }
}
