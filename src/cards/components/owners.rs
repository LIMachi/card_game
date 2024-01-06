use crate::players::{Player, MAXIMUM_PLAYERS};
use crate::utils::filter_enum::FilterEnumInserter;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Component, Serialize, Deserialize, Debug, Default, Reflect, Copy, Clone, Eq, PartialEq, Hash,
)]
#[reflect(Component)]
pub enum CardOwners {
    #[default]
    Market,
    Player(u8),
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct MarketOnwed;

impl FilterEnumInserter for CardOwners {
    fn insert(&self, entity: &mut EntityCommands) {
        entity.insert(*self);
        match self {
            Self::Market => {
                entity.insert(MarketOnwed);
            }
            Self::Player(0) => {
                entity.insert(Player::<0>);
            }
            Self::Player(1) => {
                entity.insert(Player::<1>);
            }
            Self::Player(e) => {
                panic!(
                    "invalid player id: {e}, can only go up to: {}",
                    MAXIMUM_PLAYERS - 1
                );
            }
        }
    }

    fn remove(&self, entity: &mut EntityCommands) {
        entity.remove::<Self>();
        match self {
            Self::Market => {
                entity.remove::<MarketOnwed>();
            }
            Self::Player(0) => {
                entity.remove::<Player<0>>();
            }
            Self::Player(1) => {
                entity.remove::<Player<1>>();
            }
            Self::Player(e) => {
                panic!(
                    "invalid player id: {e}, can only go up to: {}",
                    MAXIMUM_PLAYERS - 1
                );
            }
        }
    }

    fn insert_world(&self, entity: &mut EntityWorldMut) {
        entity.insert(*self);
        match self {
            Self::Market => {
                entity.insert(MarketOnwed);
            }
            Self::Player(0) => {
                entity.insert(Player::<0>);
            }
            Self::Player(1) => {
                entity.insert(Player::<1>);
            }
            Self::Player(e) => {
                panic!(
                    "invalid player id: {e}, can only go up to: {}",
                    MAXIMUM_PLAYERS - 1
                );
            }
        }
    }

    fn remove_world(&self, entity: &mut EntityWorldMut) {
        entity.remove::<Self>();
        match self {
            Self::Market => {
                entity.remove::<MarketOnwed>();
            }
            Self::Player(0) => {
                entity.remove::<Player<0>>();
            }
            Self::Player(1) => {
                entity.remove::<Player<1>>();
            }
            Self::Player(e) => {
                panic!(
                    "invalid player id: {e}, can only go up to: {}",
                    MAXIMUM_PLAYERS - 1
                );
            }
        }
    }
}

pub struct OwnersPlugin;

impl Plugin for OwnersPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CardOwners>()
            .register_type::<MarketOnwed>();
    }
}
