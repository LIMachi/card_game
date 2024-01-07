pub mod execute;
mod uniques;

use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Reflect, Clone, Eq, PartialEq)]
pub enum Action {
    Eco(u8),            //gain economy
    Atk(u8),            //gain attack
    Life(u8),           //gain life
    Draw,               //draw a card
    Discard,            //discard a card
    ScrapHand,          //scrap a card from hand
    ScrapDiscard,       //scrap a card from the discard pile
    ScrapHandOrDiscard, //Helper for the recurring optional from machine cult faction
    ScrapMarket,        //scrap a card from the market
    DestroyBase,        //target and put in discard pile target base
    EnemyDiscard,       //force targeted player to discard
    FreeBuy {
        //allow imediate aquisition of a card on the market matching the condition
        min_cost: u8,
        max_cost: u8,
        valid_kinds: Vec<CardKinds>,
    },
    NextBuyOnDeck(Vec<CardKinds>),
    #[default]
    Unique, //special action that is hard coded for a specific card, like the stealth needle ability of copying another ship
}

#[derive(Serialize, Deserialize, Debug, Default, Reflect, Clone, Eq, PartialEq)]
pub enum ActionSet {
    #[default]
    None,
    One(Action),
    Optional(Action),
    OneAndOptional(Action, Action),
    All(Vec<Action>),
    Any(Vec<Action>),
    OneOf(Action, Action),
}

#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct OnPlay(pub ActionSet, pub bool);

#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct OnScrap(pub ActionSet, pub bool);

#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct ComboBlob(pub ActionSet, pub bool);

#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct ComboMachineCult(pub ActionSet, pub bool);

#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct ComboTradeFederation(pub ActionSet, pub bool);

#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct ComboStarEmpire(pub ActionSet, pub bool);

pub struct GameActionsPlugin;

impl Plugin for GameActionsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ActionSet>()
            .register_type::<OnPlay>()
            .register_type::<OnScrap>()
            .register_type::<ComboBlob>()
            .register_type::<ComboMachineCult>()
            .register_type::<ComboTradeFederation>()
            .register_type::<ComboStarEmpire>();
    }
}
