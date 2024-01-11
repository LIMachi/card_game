use crate::utils::filter_enum::FilterEnumInserter;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::utils::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Reflect, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CardFaction {
    Blob,
    MachineCult,
    #[default]
    Neutral,
    TradeFederation,
    StarEmpire,
}

#[derive(Component, Debug, Default, Reflect, Clone, Eq, PartialEq)]
#[reflect(Component)]
pub struct CardFactions(pub HashSet<CardFaction>);

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Blob;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct MachineCult;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Neutral;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct TradeFederation;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct StarEmpire;

impl FilterEnumInserter for CardFaction {
    fn insert(&self, entity: &mut EntityCommands) {
        // entity.insert(*self);
        match self {
            Self::Blob => {
                entity.insert(Blob);
            }
            Self::MachineCult => {
                entity.insert(MachineCult);
            }
            Self::Neutral => {
                entity.insert(Neutral);
            }
            Self::TradeFederation => {
                entity.insert(TradeFederation);
            }
            Self::StarEmpire => {
                entity.insert(StarEmpire);
            }
        }
    }

    fn remove(&self, entity: &mut EntityCommands) {
        // entity.remove::<Self>();
        match self {
            Self::Blob => {
                entity.remove::<Blob>();
            }
            Self::MachineCult => {
                entity.remove::<MachineCult>();
            }
            Self::Neutral => {
                entity.remove::<Neutral>();
            }
            Self::TradeFederation => {
                entity.remove::<TradeFederation>();
            }
            Self::StarEmpire => {
                entity.remove::<StarEmpire>();
            }
        }
    }

    fn insert_world(&self, entity: &mut EntityWorldMut) {
        // entity.insert(*self);
        match self {
            Self::Blob => {
                entity.insert(Blob);
            }
            Self::MachineCult => {
                entity.insert(MachineCult);
            }
            Self::Neutral => {
                entity.insert(Neutral);
            }
            Self::TradeFederation => {
                entity.insert(TradeFederation);
            }
            Self::StarEmpire => {
                entity.insert(StarEmpire);
            }
        }
    }

    fn remove_world(&self, entity: &mut EntityWorldMut) {
        // entity.remove::<Self>();
        match self {
            Self::Blob => {
                entity.remove::<Blob>();
            }
            Self::MachineCult => {
                entity.remove::<MachineCult>();
            }
            Self::Neutral => {
                entity.remove::<Neutral>();
            }
            Self::TradeFederation => {
                entity.remove::<TradeFederation>();
            }
            Self::StarEmpire => {
                entity.remove::<StarEmpire>();
            }
        }
    }
}

pub struct FactionsPlugin;

impl Plugin for FactionsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CardFaction>()
            .register_type::<CardFactions>()
            .register_type::<Blob>()
            .register_type::<MachineCult>()
            .register_type::<Neutral>()
            .register_type::<TradeFederation>()
            .register_type::<StarEmpire>();
    }
}
