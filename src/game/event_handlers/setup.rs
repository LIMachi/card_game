use crate::cards::assets::{Deck, LoadedSet};
use crate::game::events::{GameEvent, GameEvents};
use crate::game::routines::RoutineManager;
use crate::prelude::*;
use crate::states::turn::TurnStates;

pub fn setup(
    mut commands: Commands,
    mut event: ResMut<GameEvent>,
    mut turn: ResMut<NextState<TurnStates>>,
    loaded_set: Res<LoadedSet>,
    decks: Res<Assets<Deck>>,
    mut routines: ResMut<RoutineManager>,
) {
    if let Some(GameEvents::Setup {
        seed,
        set,
        players,
        starting_player,
    }) = event.get_unprocessed()
    {
        if let Some(deck) = decks.get(&loaded_set.market_deck) {
            let mut index = 0;
            for (qty, name) in &deck.0 {
                for _ in 0..*qty {
                    let mut ec = commands.spawn((
                        CardIndex(index),
                        SpawnCard(name.clone()),
                        SpatialBundle::default(),
                        Name::new(name.clone()),
                        StartTransition {
                            owner: CardOwners::Market,
                            stack: Stacks::MarketDeck,
                            index: CardIndex(index),
                            visibility: CardVisibility::Hidden,
                            length: 0.0,
                        },
                    ));
                    CardOwners::Market.insert(&mut ec);
                    Stacks::MarketDeck.insert(&mut ec);
                    index += 1;
                }
            }
        }
        if let Some(deck) = decks.get(&loaded_set.player_deck) {
            let mut index = 0;
            for (qty, name) in &deck.0 {
                for _ in 0..*qty {
                    let mut ec = commands.spawn((
                        CardIndex(index),
                        SpawnCard(name.clone()),
                        SpatialBundle::default(),
                        Name::new(name.clone()),
                        StartTransition {
                            owner: CardOwners::Player(0),
                            stack: Stacks::PlayerDeck,
                            index: CardIndex(index),
                            visibility: CardVisibility::Hidden,
                            length: 0.0,
                        },
                    ));
                    CardOwners::Player(0).insert(&mut ec);
                    Stacks::PlayerDeck.insert(&mut ec);
                    let mut ec = commands.spawn((
                        CardIndex(index),
                        SpawnCard(name.clone()),
                        SpatialBundle::default(),
                        Name::new(name.clone()),
                        StartTransition {
                            owner: CardOwners::Player(1),
                            stack: Stacks::PlayerDeck,
                            index: CardIndex(index),
                            visibility: CardVisibility::Hidden,
                            length: 0.0,
                        },
                    ));
                    CardOwners::Player(1).insert(&mut ec);
                    Stacks::PlayerDeck.insert(&mut ec);
                    index += 1;
                }
            }
        }
        if let Some(deck) = decks.get(&loaded_set.joker_deck) {
            let mut index = 0;
            for (qty, name) in &deck.0 {
                for _ in 0..*qty {
                    let mut ec = commands.spawn((
                        CardIndex(index),
                        SpawnCard(name.clone()),
                        SpatialBundle::default(),
                        Name::new(name.clone()),
                        StartTransition {
                            owner: CardOwners::Market,
                            stack: Stacks::JokerDeck,
                            index: CardIndex(index),
                            visibility: CardVisibility::Visible,
                            length: 0.0,
                        },
                    ));
                    CardOwners::Market.insert(&mut ec);
                    Stacks::JokerDeck.insert(&mut ec);
                    index += 1;
                }
            }
        }
        routines.shuffle(CardOwners::Market, Stacks::MarketDeck, false);
        routines.shuffle(CardOwners::Player(0), Stacks::PlayerDeck, false);
        routines.shuffle(CardOwners::Player(1), Stacks::PlayerDeck, false);
        for i in 0..5 {
            routines.reload_market(i);
        }
        for i in 0..5 {
            if i >= 3 {
                if *starting_player == 0 {
                    routines.draw(1);
                } else {
                    routines.draw(0);
                }
            } else {
                routines.draw(0);
                routines.draw(1);
            }
        }
        turn.set(TurnStates::PlayerTurn(*starting_player));
        event.set_processed();
    }
}
