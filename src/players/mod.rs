use crate::cards::components::factions::CardFaction;
use crate::cards::transition::CardStateSnapshot;
use crate::prelude::{CardFactions, CardOwners, FilterEnumInserter, Stacks};
use bevy::prelude::*;
use bevy::utils::HashMap;

pub const MAXIMUM_PLAYERS: usize = 2;

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct LocalPlayer(pub u8);

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Player<const ID: u8>;

pub trait PlayerCounter {
    fn get_value(&self) -> i32;
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PlayerLife(pub i32);

impl PlayerCounter for PlayerLife {
    fn get_value(&self) -> i32 {
        self.0
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PlayerAttack(pub i32);

impl PlayerCounter for PlayerAttack {
    fn get_value(&self) -> i32 {
        self.0
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PlayerEconomy(pub i32);

impl PlayerCounter for PlayerEconomy {
    fn get_value(&self) -> i32 {
        self.0
    }
}

#[derive(Reflect, Default, Debug)]
pub struct FactionTurnTracker {
    pub bases_in_play: usize,
    pub bases_played: usize,
    pub bases_discarded: usize,
    pub bases_scrapped: usize,
    pub ships_in_play: usize,
    pub ships_played: usize,
    pub ship_discarded: usize,
    pub ships_scrapped: usize,
}

impl FactionTurnTracker {
    pub fn turn_finished(&mut self) {
        self.bases_played = 0;
        self.bases_discarded = 0;
        self.bases_scrapped = 0;
        self.ships_in_play = 0;
        self.ships_played = 0;
        self.ship_discarded = 0;
        self.ships_scrapped = 0;
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PlayerTurnTracker {
    pub common: FactionTurnTracker,
    pub cards_drawn: usize,
    pub cards_to_discard: usize,
    pub life_lost: usize,
    pub life_gained: usize,
    pub faction_counters: HashMap<CardFaction, FactionTurnTracker>,
}

impl Default for PlayerTurnTracker {
    fn default() -> Self {
        let mut faction_counters = HashMap::with_capacity(5);
        faction_counters.insert(CardFaction::Blob, FactionTurnTracker::default());
        faction_counters.insert(CardFaction::MachineCult, FactionTurnTracker::default());
        faction_counters.insert(CardFaction::Neutral, FactionTurnTracker::default());
        faction_counters.insert(CardFaction::TradeFederation, FactionTurnTracker::default());
        faction_counters.insert(CardFaction::StarEmpire, FactionTurnTracker::default());
        Self {
            common: Default::default(),
            cards_drawn: 0,
            cards_to_discard: 0,
            life_lost: 0,
            life_gained: 0,
            faction_counters,
        }
    }
}

impl PlayerTurnTracker {
    pub fn turn_finished(&mut self) {
        self.common.turn_finished();
        self.cards_drawn = 0;
        self.life_lost = 0;
        self.life_gained = 0;
        for (_, counters) in self.faction_counters.iter_mut() {
            counters.turn_finished();
        }
    }

    pub fn card_snapshots(
        &mut self,
        previous: &CardStateSnapshot,
        next: &CardStateSnapshot,
        factions: &CardFactions,
    ) {
        let was_in_play = previous.stack == Stacks::Bases || previous.stack == Stacks::UsedCards;
        let will_be_played = next.stack == Stacks::Bases || next.stack == Stacks::UsedCards;

        let base = previous.stack == Stacks::Bases || next.stack == Stacks::Bases;

        if was_in_play && !will_be_played {
            if next.stack == Stacks::DiscardPile {
                //discarded
                if base {
                    self.common.bases_discarded += 1;
                    for (faction, counters) in self.faction_counters.iter_mut() {
                        if factions.0.contains(faction) {
                            counters.bases_discarded += 1;
                        }
                    }
                } else {
                    self.common.ship_discarded += 1;
                    for (faction, counters) in self.faction_counters.iter_mut() {
                        if factions.0.contains(faction) {
                            counters.ship_discarded += 1;
                        }
                    }
                }
            }
            if next.stack == Stacks::Scrapyard {
                //scrapped
                if base {
                    self.common.bases_scrapped += 1;
                    for (faction, counters) in self.faction_counters.iter_mut() {
                        if factions.0.contains(faction) {
                            counters.bases_scrapped += 1;
                        }
                    }
                } else {
                    self.common.ships_scrapped += 1;
                    for (faction, counters) in self.faction_counters.iter_mut() {
                        if factions.0.contains(faction) {
                            counters.ships_scrapped += 1;
                        }
                    }
                }
            }
            if base {
                self.common.bases_in_play -= 1;
                for (faction, counters) in self.faction_counters.iter_mut() {
                    if factions.0.contains(faction) {
                        counters.bases_in_play -= 1;
                    }
                }
            } else {
                self.common.ships_in_play -= 1;
                for (faction, counters) in self.faction_counters.iter_mut() {
                    if factions.0.contains(faction) {
                        counters.ships_in_play -= 1;
                    }
                }
            }
        }
        if will_be_played && !was_in_play {
            if base {
                self.common.bases_in_play += 1;
                self.common.bases_played += 1;
                for (faction, counters) in self.faction_counters.iter_mut() {
                    if factions.0.contains(faction) {
                        counters.bases_played += 1;
                        counters.bases_in_play += 1;
                    }
                }
            } else {
                self.common.ships_in_play += 1;
                self.common.ships_played += 1;
                for (faction, counters) in self.faction_counters.iter_mut() {
                    if factions.0.contains(faction) {
                        counters.ships_in_play += 1;
                        counters.ships_played += 1;
                    }
                }
            }
        }
        if previous.stack == Stacks::PlayerDeck && next.stack == Stacks::Hand {
            self.cards_drawn += 1;
        }
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PlayerActionTracker {}

pub fn spawn_counters(mut commands: Commands) {
    CardOwners::Player(0).insert(&mut commands.spawn((
        Name::new(format!("Player 0 counters")),
        PlayerLife(50),
        PlayerAttack(0),
        PlayerEconomy(0),
        PlayerTurnTracker::default(),
    )));
    CardOwners::Player(1).insert(&mut commands.spawn((
        Name::new(format!("Player 1 counters")),
        PlayerLife(50),
        PlayerAttack(0),
        PlayerEconomy(0),
        PlayerTurnTracker::default(),
    )));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LocalPlayer>()
            .init_resource::<LocalPlayer>()
            .register_type::<PlayerLife>()
            .register_type::<PlayerAttack>()
            .register_type::<PlayerEconomy>()
            .register_type::<PlayerActionTracker>()
            .register_type::<FactionTurnTracker>()
            .register_type::<PlayerTurnTracker>()
            .register_type::<Player<0>>()
            .register_type::<Player<1>>()
            .add_systems(Startup, spawn_counters);
    }
}
