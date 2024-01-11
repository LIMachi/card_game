use crate::cards::actions::CardActions;
use crate::cards::transition::TransitionSystemSets;
use crate::players::{Player, PlayerTurnTracker};
use crate::prelude::*;

pub fn card_transitioning(
    mut cards: Query<
        (&mut CardActions, &mut CardTransition, &CardFactions, &Name),
        Added<CardTransition>,
    >,
    mut player_0_trackers: Query<&mut PlayerTurnTracker, (With<Player<0>>, Without<Player<1>>)>,
    mut player_1_trackers: Query<&mut PlayerTurnTracker, (With<Player<1>>, Without<Player<0>>)>,
) {
    for (mut actions, mut transition, factions, name) in cards.iter_mut() {
        if transition.next != transition.previous {
            if transition.next.stack == Stacks::DiscardPile
                || transition.next.stack == Stacks::Scrapyard
            {
                actions.reset();
            }
            match transition.previous.owner {
                CardOwners::Market => {}
                CardOwners::Player(p) => {
                    if let Ok(mut trackers) = if p == 0 {
                        player_0_trackers.get_single_mut()
                    } else {
                        player_1_trackers.get_single_mut()
                    } {
                        trackers.card_snapshots(&transition.previous, &transition.next, factions);
                    }
                }
            }
            if transition.next.owner != transition.previous.owner {
                match transition.next.owner {
                    CardOwners::Market => {}
                    CardOwners::Player(p) => {
                        if let Ok(mut trackers) = if p == 0 {
                            player_0_trackers.get_single_mut()
                        } else {
                            player_1_trackers.get_single_mut()
                        } {
                            trackers.card_snapshots(
                                &transition.previous,
                                &transition.next,
                                factions,
                            );
                        }
                    }
                }
            }
        }
    }
}

pub struct ListenersPlugin;

impl Plugin for ListenersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            card_transitioning.before(TransitionSystemSets::Update),
        );
    }
}
