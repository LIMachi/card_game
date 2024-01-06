use crate::utils::filter_enum::FilterEnumInserter;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Component, Serialize, Deserialize, Debug, Default, Reflect, Copy, Clone, Eq, PartialEq, Hash,
)]
#[reflect(Component)]
pub enum CardKinds {
    #[default]
    Ship,
    Base(i32),
    Outpost(i32),
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Ship;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Base(pub i32);

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Outpost(pub i32);

impl FilterEnumInserter for CardKinds {
    fn insert(&self, entity: &mut EntityCommands) {
        entity.insert(*self);
        match self {
            Self::Ship => {
                entity.insert(Ship);
            }
            Self::Base(life) => {
                entity.insert(Base(*life));
            }
            Self::Outpost(life) => {
                entity.insert(Outpost(*life));
            }
        }
    }

    fn remove(&self, entity: &mut EntityCommands) {
        entity.remove::<Self>();
        match self {
            Self::Ship => {
                entity.remove::<Ship>();
            }
            Self::Base(_) => {
                entity.remove::<Base>();
            }
            Self::Outpost(_) => {
                entity.remove::<Outpost>();
            }
        }
    }

    fn insert_world(&self, entity: &mut EntityWorldMut) {
        entity.insert(*self);
        match self {
            Self::Ship => {
                entity.insert(Ship);
            }
            Self::Base(life) => {
                entity.insert(Base(*life));
            }
            Self::Outpost(life) => {
                entity.insert(Outpost(*life));
            }
        }
    }

    fn remove_world(&self, entity: &mut EntityWorldMut) {
        entity.remove::<Self>();
        match self {
            Self::Ship => {
                entity.remove::<Ship>();
            }
            Self::Base(_) => {
                entity.remove::<Base>();
            }
            Self::Outpost(_) => {
                entity.remove::<Outpost>();
            }
        }
    }
}

pub struct KindsPlugin;

impl Plugin for KindsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CardKinds>()
            .register_type::<Ship>()
            .register_type::<Base>()
            .register_type::<Outpost>();
    }
}
