use crate::cards::actions::CardActions;
use crate::game::events::GameEvent;
use crate::game::routines::RoutineManager;
use crate::players::{Player, PlayerAttack, PlayerEconomy, MAXIMUM_PLAYERS};
use crate::prelude::*;
use crate::states::turn::TurnStates;

pub fn pass_turn<const PLAYER: u8>(
    mut event: ResMut<GameEvent>,
    mut turn: ResMut<NextState<TurnStates>>,
    mut player_trackers: Query<(&mut PlayerAttack, &mut PlayerEconomy), With<Player<PLAYER>>>,
    mut routines: ResMut<RoutineManager>,
    hand: Query<Entity, (With<Hand>, With<Player<PLAYER>>)>,
    used: Query<Entity, (With<UsedCards>, With<Player<PLAYER>>)>,
    mut bases: Query<
        // (
        //     Option<&mut OnPlay>,
        //     Option<&mut OnScrap>,
        //     Option<&mut ComboBlob>,
        //     Option<&mut ComboMachineCult>,
        //     Option<&mut ComboTradeFederation>,
        //     Option<&mut ComboStarEmpire>,
        // ),
        &mut CardActions,
        (With<Bases>, With<Player<PLAYER>>),
    >,
) {
    if let Ok((mut attack, mut economy)) = player_trackers.get_single_mut() {
        attack.0 = 0;
        economy.0 = 0;
    }
    // for (o0, o1, o2, o3, o4, o5) in bases.iter_mut() {
    //     if let Some(mut o) = o0 {
    //         o.1 = false;
    //     }
    //     if let Some(mut o) = o1 {
    //         o.1 = false;
    //     }
    //     if let Some(mut o) = o2 {
    //         o.1 = false;
    //     }
    //     if let Some(mut o) = o3 {
    //         o.1 = false;
    //     }
    //     if let Some(mut o) = o4 {
    //         o.1 = false;
    //     }
    //     if let Some(mut o) = o5 {
    //         o.1 = false;
    //     }
    // }
    for mut actions in bases.iter_mut() {
        actions.reset();
    }
    for card in hand.iter() {
        routines.discard(PLAYER, card);
    }
    for card in used.iter() {
        routines.discard(PLAYER, card);
    }
    for _ in 0..5 {
        routines.draw(PLAYER);
    }
    turn.set(TurnStates::PlayerTurn((PLAYER + 1) % MAXIMUM_PLAYERS as u8));
    event.set_processed();
}
