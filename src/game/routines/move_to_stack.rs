use crate::game::routines::{RoutineManager, Routines};
use crate::prelude::*;
use crate::prelude::{CardIndex, Commands, Entity, Query};

pub fn move_to_stack(
    mut commands: Commands,
    mut manager: ResMut<RoutineManager>,
    all_cards: Query<(&CardOwners, &Stacks, &CardIndex, &CardVisibility, Entity)>,
) {
    let mut finished = false;
    if let Some(Routines::PushCardToStack {
        owner: target_owner,
        stack: target_stack,
        index: target_index,
        visibility: target_visibility,
        card,
        running,
    }) = manager.routine_mut()
    {
        if all_cards.get(*card).map_or(false, |(o, s, i, v, ..)| {
            o == target_owner
                && s == target_stack
                && target_index.map_or(false, |t| t == i.0)
                && v == target_visibility
        }) {
            finished = true;
        } else if !*running {
            if target_index.is_none() {
                *target_index = Some(
                    all_cards
                        .iter()
                        .fold(0u64, |m, (_, _, v, ..)| m | 1u64 << v.0)
                        .trailing_ones() as usize,
                );
            }
            let target_index = CardIndex(target_index.unwrap());
            if let Ok((&current_owner, &current_stack, &current_index, &current_visibility, _)) =
                all_cards.get(*card)
            {
                let same_stack = current_stack == *target_stack && current_owner == *target_owner;
                for (&owner, &stack, &index, &visibility, card) in all_cards.iter() {
                    if !stack.keep_empty_spaces()
                        && index.0 > current_index.0
                        && owner == current_owner
                        && stack == current_stack
                        && !(same_stack && index.0 < target_index.0)
                    {
                        commands.entity(card).insert(StartTransition {
                            owner,
                            stack,
                            index: CardIndex(index.0 - 1),
                            visibility,
                            length: 0.5,
                        });
                    }
                    if !same_stack && owner == *target_owner && stack == *target_stack {
                        commands.entity(card).insert(StartTransition {
                            owner,
                            stack,
                            index: CardIndex(index.0 + 1),
                            visibility,
                            length: 0.5,
                        });
                    }
                }
                commands.entity(*card).insert(StartTransition {
                    owner: *target_owner,
                    stack: *target_stack,
                    index: target_index,
                    visibility: *target_visibility,
                    length: 0.5,
                });
                *running = true;
            } else {
                //no card matched the query, it might already be in a transition
                //treat this as a finish?
                finished = true;
            }
        }
    }
    if finished {
        manager.finish();
    }
}
