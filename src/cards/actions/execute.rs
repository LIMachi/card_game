use crate::cards::actions::{Action, ActionSet};
use bevy::prelude::*;

impl ActionSet {
    pub fn execute(&self, world: &mut World, card: Entity) {
        match self {
            ActionSet::None => {}
            ActionSet::One(action) => {
                action.execute(world, card);
            }
            ActionSet::Optional(action) => {
                //spawn optional button
            }
            ActionSet::OneAndOptional(action, option) => {
                action.execute(world, card);
                //spawn optional button
            }
            ActionSet::All(actions) => {
                for action in actions {
                    action.execute(world, card);
                }
            }
            ActionSet::Any(actions) => {
                //spawn choice buttons
            }
            ActionSet::OneOf(first, second) => {
                //spawn choice buttons
            }
            ActionSet::UniquePassive => {
                //TODO
            }
        }
    }
}

impl Action {
    pub fn execute(&self, world: &mut World, card: Entity) {
        match self {
            Action::Eco(qty) => {
                // if owner == CardOwners::Player(1) {
                //     if let Some(mut eco) = world.get_resource_mut::<EnemyEco>() {
                //         eco.0 += *qty as i32;
                //     }
                // } else {
                //     if let Some(mut eco) = world.get_resource_mut::<AllyEco>() {
                //         eco.0 += *qty as i32;
                //     }
                // }
            }
            Action::Atk(qty) => {
                // if owner == CardOwners::Player(1) {
                //     if let Some(mut atk) = world.get_resource_mut::<EnemyAtk>() {
                //         atk.0 += *qty as i32;
                //     }
                // } else {
                //     if let Some(mut atk) = world.get_resource_mut::<AllyAtk>() {
                //         atk.0 += *qty as i32;
                //     }
                // }
            }
            Action::Life(qty) => {
                // if owner == CardOwners::Player(1) {
                //     if let Some(mut life) = world.get_resource_mut::<EnemyLife>() {
                //         life.0 += *qty as i32;
                //     }
                // } else {
                //     if let Some(mut life) = world.get_resource_mut::<AllyLife>() {
                //         life.0 += *qty as i32;
                //     }
                // }
            }
            Action::Draw => {
                // if let Some(mut actions) = world.get_resource_mut::<GameActions>() {
                //     actions.push(false, GameAction::Draw { owner });
                // }
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
