use crate::cards::actions::KindMask;
use crate::game::routines::{RoutineManager, Routines};
use crate::prelude::*;
use crate::prelude::{CardIndex, Commands, Entity, Query};

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct NextBuyOnDeckFlag(pub KindMask); //FIXME: player counter reset on turn end?

impl Default for NextBuyOnDeckFlag {
    fn default() -> Self {
        Self(KindMask::None)
    }
}

pub fn move_to_stack(
    mut commands: Commands,
    mut manager: ResMut<RoutineManager>,
    all_cards: Query<(
        &CardOwners,
        &Stacks,
        &CardIndex,
        &CardVisibility,
        Entity,
        &Name,
        &CardKinds,
    )>,
    mut next_buy_on_deck_flag: ResMut<NextBuyOnDeckFlag>,
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
            if let Ok((
                &current_owner,
                &current_stack,
                &current_index,
                &current_visibility,
                _,
                name,
                &kind,
            )) = all_cards.get(*card)
            {
                if *target_stack == Stacks::Scrapyard && name.as_str() == "Explorer" {
                    *target_stack = Stacks::JokerDeck;
                }
                if *target_stack == Stacks::DiscardPile
                    && current_stack == Stacks::MarketRow
                    && kind.in_mask(next_buy_on_deck_flag.0)
                {
                    next_buy_on_deck_flag.0 = KindMask::None;
                    *target_stack = Stacks::PlayerDeck;
                }
                if target_index.is_none() {
                    *target_index = Some(
                        all_cards
                            .iter()
                            .fold(0u64, |m, (_, _, v, ..)| m | 1u64 << v.0)
                            .trailing_ones() as usize,
                    );
                }
                let target_index = CardIndex(target_index.unwrap());

                let same_stack = current_stack == *target_stack && current_owner == *target_owner;
                for (&owner, &stack, &index, &visibility, card, ..) in all_cards.iter() {
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
