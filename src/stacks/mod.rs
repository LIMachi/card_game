use crate::utils::filter_enum::FilterEnumInserter;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Component, Serialize, Deserialize, Debug, Default, Reflect, Copy, Clone, Eq, PartialEq, Hash,
)]
#[reflect(Component)]
pub enum Stacks {
    #[default]
    MarketDeck,
    MarketRow,
    JokerDeck,
    Scrapyard,
    PlayerDeck,
    Hand,
    UsedCards,
    Bases,
    DiscardPile,
    Focused,
    Selection,
    Log,
}

impl Stacks {
    pub fn keep_empty_spaces(&self) -> bool {
        match self {
            Stacks::MarketDeck => false,
            Stacks::MarketRow => true,
            Stacks::JokerDeck => false,
            Stacks::Scrapyard => false,
            Stacks::PlayerDeck => false,
            Stacks::Hand => true,
            Stacks::UsedCards => true,
            Stacks::Bases => false,
            Stacks::DiscardPile => false,
            Stacks::Focused => false,
            Stacks::Selection => true,
            Stacks::Log => true,
        }
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct MarketDeck;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct MarketRow;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct JokerDeck;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Scrapyard;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PlayerDeck;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Hand;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct UsedCards;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Bases;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct DiscardPile;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Focused;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Selection;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Log;

impl FilterEnumInserter for Stacks {
    fn insert(&self, entity: &mut EntityCommands) {
        entity.insert(*self);
        match self {
            Self::MarketDeck => {
                entity.insert(MarketDeck);
            }
            Self::MarketRow => {
                entity.insert(MarketRow);
            }
            Self::JokerDeck => {
                entity.insert(JokerDeck);
            }
            Self::Scrapyard => {
                entity.insert(Scrapyard);
            }
            Self::PlayerDeck => {
                entity.insert(PlayerDeck);
            }
            Self::Hand => {
                entity.insert(Hand);
            }
            Self::UsedCards => {
                entity.insert(UsedCards);
            }
            Self::Bases => {
                entity.insert(Bases);
            }
            Self::DiscardPile => {
                entity.insert(DiscardPile);
            }
            Self::Focused => {
                entity.insert(Focused);
            }
            Self::Selection => {
                entity.insert(Selection);
            }
            Self::Log => {
                entity.insert(Log);
            }
        }
    }

    fn remove(&self, entity: &mut EntityCommands) {
        entity.remove::<Self>();
        match self {
            Self::MarketDeck => {
                entity.remove::<MarketDeck>();
            }
            Self::MarketRow => {
                entity.remove::<MarketRow>();
            }
            Self::JokerDeck => {
                entity.remove::<JokerDeck>();
            }
            Self::Scrapyard => {
                entity.remove::<Scrapyard>();
            }
            Self::PlayerDeck => {
                entity.remove::<PlayerDeck>();
            }
            Self::Hand => {
                entity.remove::<Hand>();
            }
            Self::UsedCards => {
                entity.remove::<UsedCards>();
            }
            Self::Bases => {
                entity.remove::<Bases>();
            }
            Self::DiscardPile => {
                entity.remove::<DiscardPile>();
            }
            Self::Focused => {
                entity.remove::<Focused>();
            }
            Self::Selection => {
                entity.remove::<Selection>();
            }
            Self::Log => {
                entity.remove::<Log>();
            }
        }
    }

    fn insert_world(&self, entity: &mut EntityWorldMut) {
        entity.insert(*self);
        match self {
            Self::MarketDeck => {
                entity.insert(MarketDeck);
            }
            Self::MarketRow => {
                entity.insert(MarketRow);
            }
            Self::JokerDeck => {
                entity.insert(JokerDeck);
            }
            Self::Scrapyard => {
                entity.insert(Scrapyard);
            }
            Self::PlayerDeck => {
                entity.insert(PlayerDeck);
            }
            Self::Hand => {
                entity.insert(Hand);
            }
            Self::UsedCards => {
                entity.insert(UsedCards);
            }
            Self::Bases => {
                entity.insert(Bases);
            }
            Self::DiscardPile => {
                entity.insert(DiscardPile);
            }
            Self::Focused => {
                entity.insert(Focused);
            }
            Self::Selection => {
                entity.insert(Selection);
            }
            Self::Log => {
                entity.insert(Log);
            }
        }
    }

    fn remove_world(&self, entity: &mut EntityWorldMut) {
        entity.remove::<Self>();
        match self {
            Self::MarketDeck => {
                entity.remove::<MarketDeck>();
            }
            Self::MarketRow => {
                entity.remove::<MarketRow>();
            }
            Self::JokerDeck => {
                entity.remove::<JokerDeck>();
            }
            Self::Scrapyard => {
                entity.remove::<Scrapyard>();
            }
            Self::PlayerDeck => {
                entity.remove::<PlayerDeck>();
            }
            Self::Hand => {
                entity.remove::<Hand>();
            }
            Self::UsedCards => {
                entity.remove::<UsedCards>();
            }
            Self::Bases => {
                entity.remove::<Bases>();
            }
            Self::DiscardPile => {
                entity.remove::<DiscardPile>();
            }
            Self::Focused => {
                entity.remove::<Focused>();
            }
            Self::Selection => {
                entity.remove::<Selection>();
            }
            Self::Log => {
                entity.remove::<Log>();
            }
        }
    }
}

pub struct StacksPlugin;

impl Plugin for StacksPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Stacks>()
            .register_type::<MarketDeck>()
            .register_type::<MarketRow>()
            .register_type::<JokerDeck>()
            .register_type::<Scrapyard>()
            .register_type::<PlayerDeck>()
            .register_type::<Hand>()
            .register_type::<UsedCards>()
            .register_type::<Bases>()
            .register_type::<DiscardPile>()
            .register_type::<Focused>()
            .register_type::<Selection>()
            .register_type::<Log>();
    }
}
