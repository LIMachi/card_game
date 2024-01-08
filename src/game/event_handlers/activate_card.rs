use crate::cards::actions::CardActions;
use crate::game::events::{GameEvent, GameEvents};
use crate::game::routines::RoutineManager;
use crate::players::Player;
use crate::prelude::*;
use crate::states::turn::TurnStates;
use bevy::utils::HashSet;

pub fn activate_card<const PLAYER: u8>(
    mut events: ResMut<GameEvent>,
    mut routines: ResMut<RoutineManager>,
    turn: Res<State<TurnStates>>,
    used_cards: Query<(Entity, &CardIndex), (With<Player<PLAYER>>, With<UsedCards>)>,
    bases: Query<(Entity, &CardIndex), (With<Player<PLAYER>>, With<Bases>)>,
    mut card_actions: Query<&mut CardActions, With<Player<PLAYER>>>,
    blobs: Query<
        Entity,
        (
            With<Player<PLAYER>>,
            Or<(With<UsedCards>, With<Bases>)>,
            With<Blob>,
        ),
    >,
    machine_cults: Query<
        Entity,
        (
            With<Player<PLAYER>>,
            Or<(With<UsedCards>, With<Bases>)>,
            With<MachineCult>,
        ),
    >,
    trade_federations: Query<
        Entity,
        (
            With<Player<PLAYER>>,
            Or<(With<UsedCards>, With<Bases>)>,
            With<TradeFederation>,
        ),
    >,
    star_empires: Query<
        Entity,
        (
            With<Player<PLAYER>>,
            Or<(With<UsedCards>, With<Bases>)>,
            With<StarEmpire>,
        ),
    >,
) {
    if let Some(&GameEvents::ActivateCard {
        base,
        index,
        action,
    }) = events.get_unprocessed()
    {
        if let TurnStates::PlayerTurn(p) = turn.get() {
            if *p != PLAYER {
                return;
            }
            let mut card = Entity::PLACEHOLDER;
            if base {
                for (e, i) in bases.iter() {
                    if i.0 == index as usize {
                        card = e;
                    }
                }
            } else {
                for (e, i) in used_cards.iter() {
                    if i.0 == index as usize {
                        card = e;
                    }
                }
            }
            let mut ok = false;
            if card != Entity::PLACEHOLDER {
                if let Ok(mut ca) = card_actions.get_mut(card) {
                    if blobs.iter().count() > 1 {
                        ca.add_ally(CardFactions::Blob);
                    }
                    if trade_federations.iter().count() > 1 {
                        ca.add_ally(CardFactions::TradeFederation);
                    }
                    if machine_cults.iter().count() > 1 {
                        ca.add_ally(CardFactions::MachineCult);
                    }
                    if star_empires.iter().count() > 1 {
                        ca.add_ally(CardFactions::StarEmpire);
                    }
                    if let Some(set) = ca.use_by_index(action) {
                        routines.activate_card(card, set.clone());
                        ok = true;
                    }
                }
            }
            if ok {
                events.set_processed();
            } else {
                events.cancel();
            }
        }
    }
}
