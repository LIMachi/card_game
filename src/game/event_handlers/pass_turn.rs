use crate::cards::actions::CardActions;
use crate::cards::components::kinds::BaseLife;
use crate::game::events::GameEvent;
use crate::game::routines::RoutineManager;
use crate::players::{Player, PlayerAttack, PlayerEconomy, PlayerTurnTracker, MAXIMUM_PLAYERS};
use crate::prelude::*;
use crate::states::turn::TurnStates;

pub fn pass_turn<const PLAYER: u8>(
    mut event: ResMut<GameEvent>,
    mut turn: ResMut<NextState<TurnStates>>,
    mut player_trackers: Query<
        (
            &mut PlayerAttack,
            &mut PlayerEconomy,
            &mut PlayerTurnTracker,
        ),
        With<Player<PLAYER>>,
    >,
    mut routines: ResMut<RoutineManager>,
    hand: Query<Entity, (With<Hand>, With<Player<PLAYER>>)>,
    used: Query<Entity, (With<UsedCards>, With<Player<PLAYER>>)>,
    mut bases_actions: Query<&mut CardActions, (With<Bases>, With<Player<PLAYER>>)>,
    mut bases_life: Query<
        (&mut BaseLife, Option<&Base>, Option<&Outpost>),
        (Or<(With<Base>, With<Outpost>)>, With<Player<PLAYER>>),
    >,
) {
    if let Ok((mut attack, mut economy, mut trackers)) = player_trackers.get_single_mut() {
        attack.0 = 0;
        economy.0 = 0;
        trackers.turn_finished();
    }
    for mut actions in bases_actions.iter_mut() {
        actions.reset();
    }
    for (mut life, base, outpost) in bases_life.iter_mut() {
        if let Some(Base(v)) = base {
            life.0 = *v;
        }
        if let Some(Outpost(v)) = outpost {
            life.0 = *v;
        }
    }
    for card in hand.iter() {
        routines.discard(PLAYER, card);
    }
    for card in used.iter() {
        routines.discard(PLAYER, card);
    }
    for _ in 0..5 {
        routines.draw(PLAYER, false);
    }
    turn.set(TurnStates::PlayerTurn((PLAYER + 1) % MAXIMUM_PLAYERS as u8));
    event.set_processed();
}
