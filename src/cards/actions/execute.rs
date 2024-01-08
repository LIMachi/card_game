use super::Action;
use crate::game::routines::RoutineManager;
use crate::players::{Player, PlayerAttack, PlayerEconomy, PlayerLife};
use crate::prelude::*;

impl Action {
    pub fn execute(&self, world: &mut World, card: Entity) {
        if let Some(&CardOwners::Player(owner)) = world.get::<CardOwners>(card) {
            match self {
                Action::Eco(qty) => {
                    if owner == 0 {
                        if let Ok(mut eco) = world
                            .query_filtered::<&mut PlayerEconomy, With<Player<0>>>()
                            .get_single_mut(world)
                        {
                            eco.0 += *qty as i32;
                        }
                    }
                    if owner == 1 {
                        if let Ok(mut eco) = world
                            .query_filtered::<&mut PlayerEconomy, With<Player<1>>>()
                            .get_single_mut(world)
                        {
                            eco.0 += *qty as i32;
                        }
                    }
                }
                Action::Atk(qty) => {
                    if owner == 0 {
                        if let Ok(mut atk) = world
                            .query_filtered::<&mut PlayerAttack, With<Player<0>>>()
                            .get_single_mut(world)
                        {
                            atk.0 += *qty as i32;
                        }
                    }
                    if owner == 1 {
                        if let Ok(mut atk) = world
                            .query_filtered::<&mut PlayerAttack, With<Player<1>>>()
                            .get_single_mut(world)
                        {
                            atk.0 += *qty as i32;
                        }
                    }
                }
                Action::Life(qty) => {
                    if owner == 0 {
                        if let Ok(mut life) = world
                            .query_filtered::<&mut PlayerLife, With<Player<0>>>()
                            .get_single_mut(world)
                        {
                            life.0 += *qty as i32;
                        }
                    }
                    if owner == 1 {
                        if let Ok(mut life) = world
                            .query_filtered::<&mut PlayerLife, With<Player<1>>>()
                            .get_single_mut(world)
                        {
                            life.0 += *qty as i32;
                        }
                    }
                }
                Action::Draw(qty) => {
                    if let Some(mut routines) = world.get_resource_mut::<RoutineManager>() {
                        for _ in 0..*qty {
                            routines.draw(owner);
                        }
                    }
                }
                Action::Discard => {
                    // if let Some(mut actions) = world.get_resource_mut::<Events<GameActions>>() {
                    //     actions.send(GameActions::discard(card, pos.is_enemy()));
                    // }
                }
                Action::ScrapHand => {}
                Action::ScrapDiscard => {}
                Action::ScrapHandOrDiscard => {}
                Action::ScrapMarket => {}
                Action::DestroyBase => {}
                Action::EnemyDiscard => {
                    // if let Some(mut actions) = world.get_resource_mut::<Events<GameActions>>() {
                    // actions.send(GameActions::discard(card, !pos.is_enemy()));
                    // }
                }
                Action::FreeBuy {
                    min_cost,
                    max_cost,
                    valid_kinds,
                } => {}
                Action::NextBuyOnDeck(kinds) => {}
                Action::Unique => {}
            }
        }
    }
}
