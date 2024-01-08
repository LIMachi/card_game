pub mod execute;
mod uniques;

use crate::cards::assets::Card;
use crate::prelude::*;
use bevy::utils::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Write};

#[derive(Serialize, Deserialize, Debug, Default, Reflect, Clone, Eq, PartialEq)]
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

//new format for actions: an action set is associated to a condition
//viper:
//- before:
//play: One(Atk(1)),
//scrap: None,
//combo: {}
//- after:
//actions: [(None, One(Atk(1)))]

#[derive(Serialize, Deserialize, Debug, Default, Reflect, Clone, Eq, PartialEq)]
pub enum Condition {
    #[default]
    None,
    Scrap,
    Ally(CardFactions),
    DoubleAlly(CardFactions),
    TwoBases,
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
            Action::Unique => {
                //special formating for this one :)
                //FIXME
                f.write_str("TODO: unique")
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Reflect, Clone, Eq, PartialEq)]
pub enum ActionSet {
    #[default]
    None,
    // One(Vec<Action>),
    // Multi(Vec<Action>),
    One(Action),
    Optional(Action),
    OneAndOptional(Action, Action),
    All(Vec<Action>),
    Any(Vec<Action>),
    OneOf(Action, Action),
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
            ActionSet::All(actions) => {
                for i in 0..actions.len() - 1 {
                    f.write_fmt(format_args!("{}, ", actions[i]))?;
                }
                f.write_fmt(format_args!("{}.", actions[actions.len() - 1]))
            }
            ActionSet::Any(actions) => {
                f.write_fmt(format_args!("you may {} and/or ", actions[0]))?;
                for i in 1..actions.len() - 1 {
                    f.write_fmt(format_args!("{} and/or ", actions[i]))?;
                }
                f.write_fmt(format_args!("{}.", actions[actions.len() - 1]))
            }
            ActionSet::OneOf(first, second) => f.write_fmt(format_args!("{first} or {second}.")),
        }
    }
}

impl ActionSet {
    //bool: use preceding "you may" msg + "do nothing" option at the end
    //string: separator between each element
    pub fn option_separator(&self) -> Option<(bool, &str)> {
        match self {
            ActionSet::None => None,
            ActionSet::One(_) => None,
            ActionSet::Optional(_) => Some((true, "")),
            ActionSet::OneAndOptional(_, _) => Some((true, "")),
            ActionSet::All(_) => None,
            ActionSet::Any(_) => Some((true, "and/or")),
            ActionSet::OneOf(_, _) => Some((false, "or")),
        }
    }
}

// #[derive(Component, Reflect, Default, Debug, Clone)]
// #[reflect(Component)]
// pub struct OnPlay(pub ActionSet, pub bool);
//
// #[derive(Component, Reflect, Default, Debug, Clone)]
// #[reflect(Component)]
// pub struct OnScrap(pub ActionSet, pub bool);
//
// #[derive(Component, Reflect, Default, Debug, Clone)]
// #[reflect(Component)]
// pub struct ComboBlob(pub ActionSet, pub bool);
//
// #[derive(Component, Reflect, Default, Debug, Clone)]
// #[reflect(Component)]
// pub struct ComboMachineCult(pub ActionSet, pub bool);
//
// #[derive(Component, Reflect, Default, Debug, Clone)]
// #[reflect(Component)]
// pub struct ComboTradeFederation(pub ActionSet, pub bool);
//
// #[derive(Component, Reflect, Default, Debug, Clone)]
// #[reflect(Component)]
// pub struct ComboStarEmpire(pub ActionSet, pub bool);

#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct CardActions {
    on_play: (ActionSet, bool),
    on_scrap: (ActionSet, bool),
    combos: HashMap<CardFactions, (bool, ActionSet, bool)>,
}

impl CardActions {
    pub fn add_ally(&mut self, ally: CardFactions) {
        if let Some((allied, ..)) = self.combos.get_mut(&ally) {
            *allied = true;
        }
    }

    pub fn count(&self) -> u8 {
        let play = if self.on_play.0 != ActionSet::None {
            1
        } else {
            0
        };
        let scrap = if self.on_scrap.0 != ActionSet::None {
            1
        } else {
            0
        };
        self.combos.len() as u8 + play + scrap
    }

    pub fn from_serialized_card(card: &Card) -> Self {
        let mut combos = HashMap::with_capacity(card.combo.len());
        for (faction, set) in card.combo.iter() {
            combos.insert(*faction, (false, set.clone(), false));
        }
        Self {
            on_play: (card.play.clone(), false),
            on_scrap: (card.scrap.clone(), false),
            combos,
        }
    }

    pub fn reset(&mut self) {
        self.on_play.1 = false;
        self.on_scrap.1 = false;
        for (_, (allied, _, used)) in self.combos.iter_mut() {
            *used = false;
            *allied = false;
        }
    }

    pub fn play_action_available(&self) -> bool {
        self.on_play.0 != ActionSet::None && !self.on_play.1
    }

    pub fn scrap_action_available(&self) -> bool {
        self.on_scrap.0 != ActionSet::None && !self.on_scrap.1
    }

    pub fn combo_action_available(&self, with: CardFactions) -> bool {
        self.combos.get(&with).map_or(false, |(allied, set, used)| {
            *allied && *set != ActionSet::None && !*used
        })
    }

    pub fn peek_play_action(&self) -> Option<&ActionSet> {
        if self.play_action_available() {
            Some(&self.on_play.0)
        } else {
            None
        }
    }

    pub fn use_play_action(&mut self) -> Option<&ActionSet> {
        if self.play_action_available() {
            self.on_play.1 = true;
            Some(&self.on_play.0)
        } else {
            None
        }
    }

    pub fn peek_scrap_action(&self) -> Option<&ActionSet> {
        if self.scrap_action_available() {
            Some(&self.on_scrap.0)
        } else {
            None
        }
    }

    pub fn use_scrap_action(&mut self) -> Option<&ActionSet> {
        if self.scrap_action_available() {
            self.on_scrap.1 = true;
            Some(&self.on_scrap.0)
        } else {
            None
        }
    }

    pub fn peek_combo_action(&self, with: CardFactions) -> Option<&ActionSet> {
        self.combos.get(&with).and_then(|(allied, set, used)| {
            if *allied && *set != ActionSet::None && !*used {
                Some(&*set)
            } else {
                None
            }
        })
    }

    pub fn use_combo_action(&mut self, with: CardFactions) -> Option<&ActionSet> {
        self.combos.get_mut(&with).and_then(|(allied, set, used)| {
            if *allied && *set != ActionSet::None && !*used {
                *used = true;
                Some(&*set) //&* used to remove mutability
            } else {
                None
            }
        })
    }

    pub fn peek_by_index(&self, mut index: u8) -> Option<&ActionSet> {
        if self.on_play.0 != ActionSet::None {
            if index == 0 {
                return if !self.on_play.1 {
                    Some(&self.on_play.0)
                } else {
                    None
                };
            } else {
                index -= 1;
            }
        }
        if self.on_scrap.0 != ActionSet::None {
            if index == 0 {
                return if !self.on_scrap.1 {
                    Some(&self.on_scrap.0)
                } else {
                    None
                };
            } else {
                index -= 1;
            }
        }
        for (_, (allied, actions, used)) in self.combos.iter() {
            if index == 0 {
                return if *allied && *actions != ActionSet::None && !*used {
                    Some(actions)
                } else {
                    None
                };
            } else {
                index -= 1;
            }
        }
        None
    }

    pub fn use_by_index(&mut self, mut index: u8) -> Option<&ActionSet> {
        if self.on_play.0 != ActionSet::None {
            if index == 0 {
                return if !self.on_play.1 {
                    self.on_play.1 = true;
                    Some(&self.on_play.0)
                } else {
                    None
                };
            } else {
                index -= 1;
            }
        }
        if self.on_scrap.0 != ActionSet::None {
            if index == 0 {
                return if !self.on_scrap.1 {
                    self.on_scrap.1 = true;
                    Some(&self.on_scrap.0)
                } else {
                    None
                };
            } else {
                index -= 1;
            }
        }
        for (_, (allied, actions, used)) in self.combos.iter_mut() {
            if index == 0 {
                return if *allied && *actions != ActionSet::None && !*used {
                    *used = true;
                    Some(&*actions) //&* used to remove mutability
                } else {
                    None
                };
            } else {
                index -= 1;
            }
        }
        None
    }
}

pub struct GameActionsPlugin;

impl Plugin for GameActionsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Action>()
            .register_type::<ActionSet>()
            // .register_type::<OnPlay>()
            // .register_type::<OnScrap>()
            // .register_type::<ComboBlob>()
            // .register_type::<ComboMachineCult>()
            // .register_type::<ComboTradeFederation>()
            // .register_type::<ComboStarEmpire>()
            .register_type::<CardActions>();
    }
}
