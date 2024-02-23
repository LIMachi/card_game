mod uniques;

use crate::cards::actions::uniques::Uniques;
use crate::cards::assets::Card;
use crate::cards::components::factions::CardFaction;
use crate::players::PlayerTurnTracker;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter, Write};

#[derive(Serialize, Deserialize, Debug, Default, Reflect, Copy, Clone, Eq, PartialEq)]
pub enum KindMask {
    #[default]
    Any = 0b111,
    Ships = 0b001,
    Bases = 0b010,
    Outposts = 0b100,
    NonShip = 0b110,
    NonBase = 0b101,
    NonOutpost = 0b011,
    None = 0,
}

#[derive(Serialize, Deserialize, Debug, Reflect, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    Eco(u8),            //gain economy
    Atk(u8),            //gain attack
    Life(u8),           //gain life
    Draw(u8),           //draw x cards
    Discard,            //discard a card
    ScrapHand,          //scrap a card from hand
    ScrapDiscard,       //scrap a card from the discard pile
    ScrapHandOrDiscard, //Helper for the recurring optional from machine cult faction
    ScrapMarket,        //scrap a card from the market
    ScrapSelf, //scrap the card that triggered this action (not to confuse with the scrap condition, this can be used as part of Do/Ally action)
    DestroyBase, //target and put in discard pile target base
    EnemyDiscard, //force targeted player to discard
    FreeBuy {
        //allow imediate aquisition of a card on the market matching the condition
        min_cost: u8,
        max_cost: u8,
        valid_kinds: KindMask,
    },
    NextBuyOnDeck(KindMask),
    Unique(Uniques), //special action that is hard coded for a specific card, like the stealth needle ability of copying another ship
}

impl Default for Action {
    fn default() -> Self {
        Self::Unique(Uniques::Unimplemented)
    }
}

//new format for actions: an action set is associated to a condition
//viper:
//- before:
//play: One(Atk(1)),
//scrap: None,
//combo: {}
//- after:
//actions: [(None, One(Atk(1)))]

#[derive(Serialize, Deserialize, Debug, Default, Reflect, Copy, Clone, Eq, PartialEq)]
pub enum ActionCondition {
    #[default]
    None,
    Do(ActionSet),
    Scrap(ActionSet),
    Ally(CardFaction, ActionSet),
    DoubleAlly(CardFaction, CardFaction, ActionSet),
}

impl ActionCondition {
    pub fn check(&self, tracker: &PlayerTurnTracker) -> bool {
        match self {
            Self::None => false,
            Self::Do(_) => true,
            Self::Scrap(_) => true,
            Self::Ally(faction, _) => tracker
                .faction_counters
                .get(faction)
                .map_or(false, |c| c.ships_in_play + c.bases_in_play > 1),
            Self::DoubleAlly(first, second, _) => {
                if first == second {
                    tracker
                        .faction_counters
                        .get(first)
                        .map_or(false, |c| c.ships_in_play + c.bases_in_play > 2)
                } else {
                    tracker
                        .faction_counters
                        .get(first)
                        .map_or(false, |c| c.ships_in_play + c.bases_in_play > 1)
                        && tracker
                            .faction_counters
                            .get(second)
                            .map_or(false, |c| c.ships_in_play + c.bases_in_play > 1)
                }
            }
        }
    }

    pub fn get_action_set(&self) -> Option<(&ActionSet, bool)> {
        match self {
            ActionCondition::None => None,
            ActionCondition::Do(set) => Some((set, false)),
            ActionCondition::Scrap(set) => Some((set, true)),
            ActionCondition::Ally(_, set) => Some((set, false)),
            ActionCondition::DoubleAlly(_, _, set) => Some((set, false)),
        }
    }

    pub fn execute(&self, world: &mut World, index: u8, card: Entity) {
        match self {
            ActionCondition::None => {}
            ActionCondition::Do(set) => {
                // set.execute(world, index, card);
            }
            ActionCondition::Scrap(set) => {
                // set.execute(world, index, card);
            }
            ActionCondition::Ally(_, set) => {
                // set.execute(world, index, card);
            }
            ActionCondition::DoubleAlly(_, _, set) => {
                // set.execute(world, index, card);
            }
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Eco(qty) => f.write_fmt(format_args!("gain {qty} economy")),
            Action::Atk(qty) => f.write_fmt(format_args!("gain {qty} attack")),
            Action::Life(qty) => f.write_fmt(format_args!("gain {qty} life")),
            Action::Draw(qty) => {
                if *qty == 1 {
                    f.write_str("draw a card")
                } else {
                    f.write_fmt(format_args!("draw {qty} cards"))
                }
            }
            Action::Discard => f.write_str("discard a card"),
            Action::ScrapHand => f.write_str("scrap a card in your hand"),
            Action::ScrapDiscard => f.write_str("scrap a card in your discard pile"),
            Action::ScrapHandOrDiscard => f.write_str("scrap a card in your hand or discard pile"),
            Action::ScrapMarket => f.write_str("scrap a card in the trade row"),
            Action::DestroyBase => f.write_str("destroy target base"),
            Action::EnemyDiscard => f.write_str("target opponent discard a card"),
            Action::FreeBuy {
                min_cost,
                max_cost,
                valid_kinds,
            } => {
                //special formating for this one :)
                //FIXME
                f.write_str("TODO: free buy")
            }
            Action::NextBuyOnDeck(valid_kinds) => {
                //special formating for this one :)
                //FIXME
                let kind = "ship";
                f.write_fmt(format_args!(
                    "put the next {kind} you acquire this turn on top of your deck"
                ))
            }
            Action::Unique(unique) => std::fmt::Display::fmt(&unique, f),
            Action::ScrapSelf => f.write_str("TODO: scrap self"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Reflect, Copy, Clone, Eq, PartialEq)]
pub enum ActionSet {
    #[default]
    None,
    // One(Vec<Action>),
    // Multi(Vec<Action>),
    One(Action),
    Optional(Action),
    OneAndOptional(Action, Action),
    Two(Action, Action),
    Three(Action, Action, Action),
    Four(Action, Action, Action, Action),
    AnyOf2(Action, Action),
    OneOf2(Action, Action),
}

impl Display for ActionSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionSet::None => f.write_str("BUG: empty action set"),
            ActionSet::One(action) => f.write_fmt(format_args!("{action}.")),
            ActionSet::Optional(option) => f.write_fmt(format_args!("you may {option}.")),
            ActionSet::OneAndOptional(action, option) => {
                f.write_fmt(format_args!("{action}, you may {option}."))
            }
            ActionSet::Two(first, second) => f.write_fmt(format_args!("{first}, {second}.")),
            ActionSet::Three(first, second, third) => {
                f.write_fmt(format_args!("{first}, {second}, {third}."))
            }
            ActionSet::Four(first, second, third, fourth) => {
                f.write_fmt(format_args!("{first}, {second}, {third}, {fourth}.",))
            }
            ActionSet::AnyOf2(first, second) => {
                f.write_fmt(format_args!("you may {first} and/or {second}."))
            }
            ActionSet::OneOf2(first, second) => f.write_fmt(format_args!("{first} or {second}.")),
        }
    }
}

#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct CardActions {
    actions: Vec<(ActionCondition, bool)>,
}

impl CardActions {
    pub fn len(&self) -> u8 {
        self.actions.len() as u8
    }

    pub fn from_serialized_card(card: &Card) -> Self {
        Self {
            actions: card.actions.iter().map(|c| (c.clone(), false)).collect(),
        }
    }

    pub fn reset(&mut self) {
        for (_, used) in self.actions.iter_mut() {
            *used = false;
        }
    }

    pub fn is_action_real(&self, index: u8) -> bool {
        self.actions
            .get(index as usize)
            .map_or(false, |(c, _)| c != &ActionCondition::None)
    }

    pub fn is_action_available(&self, index: u8, tracker: &PlayerTurnTracker) -> bool {
        if let Some((condition, used)) = self.actions.get(index as usize) {
            !*used && condition.check(tracker)
        } else {
            false
        }
    }

    pub fn use_action(
        &mut self,
        index: u8,
        tracker: &PlayerTurnTracker,
    ) -> Option<(&ActionSet, bool)> {
        if let Some((condition, used)) = self.actions.get_mut(index as usize) {
            if !*used && condition.check(tracker) {
                *used = true;
                condition.get_action_set()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn execute(
        &mut self,
        world: &mut World,
        index: u8,
        card: Entity,
        tracker: &PlayerTurnTracker,
    ) {
        if let Some((condition, used)) = self.actions.get_mut(index as usize) {
            if !*used && condition.check(tracker) {
                *used = true;
                condition.execute(world, index, card);
            }
        }
    }
}

pub struct GameActionsPlugin;

impl Plugin for GameActionsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Action>()
            .register_type::<ActionSet>()
            .register_type::<ActionCondition>()
            .register_type::<CardActions>();
    }
}
