use crate::cards::transition::PositionGenerator;
use crate::prelude::*;
use bevy::utils::HashMap;

impl Default for TransitionTransforms {
    fn default() -> Self {
        let mut positions = HashMap::new();

        positions.insert(
            (CardOwners::Market, Stacks::MarketDeck),
            PositionGenerator {
                root: Vec3::new(32., 0., 0.),
                index_offset: Vec3::new(0., -CARD_DEPTH, 0.),
                inverted_indexes: true,
                keep_base_vertical: true,
            },
        );
        positions.insert(
            (CardOwners::Market, Stacks::MarketRow),
            PositionGenerator {
                root: Vec3::new(23.5, 0., 0.),
                index_offset: Vec3::new(-CARD_WIDTH * 1.4, 0., 0.),
                inverted_indexes: false,
                keep_base_vertical: false,
            },
        );
        positions.insert(
            (CardOwners::Market, Stacks::JokerDeck),
            PositionGenerator {
                root: Vec3::new(-22., 0., 0.),
                index_offset: Vec3::new(0., -CARD_DEPTH, 0.),
                inverted_indexes: true,
                keep_base_vertical: true,
            },
        );
        positions.insert(
            (CardOwners::Market, Stacks::Scrapyard),
            PositionGenerator {
                root: Vec3::new(-32., 0., 0.),
                index_offset: Vec3::new(0., -CARD_DEPTH, 0.),
                inverted_indexes: true,
                keep_base_vertical: false,
            },
        );
        positions.insert(
            (CardOwners::Market, Stacks::Focused),
            PositionGenerator {
                root: Vec3::new(0., -30., 0.),
                index_offset: Vec3::new(0., 0., 0.),
                inverted_indexes: false,
                keep_base_vertical: false,
            },
        );

        for p in 0..=1 {
            positions.insert(
                (CardOwners::Player(p), Stacks::PlayerDeck),
                PositionGenerator {
                    root: Vec3::new(32., 0., if p == 0 { -15. } else { 15. }),
                    index_offset: Vec3::new(0., -CARD_DEPTH, 0.),
                    inverted_indexes: true,
                    keep_base_vertical: true,
                },
            );
            positions.insert(
                (CardOwners::Player(p), Stacks::Hand),
                PositionGenerator {
                    root: Vec3::new(23.5, 0., if p == 0 { -15. } else { 15. }),
                    index_offset: Vec3::new(-CARD_WIDTH * 1.4, 0., 0.),
                    inverted_indexes: false,
                    keep_base_vertical: false,
                },
            );
            positions.insert(
                (CardOwners::Player(p), Stacks::UsedCards),
                PositionGenerator {
                    root: Vec3::new(23.5, CARD_DEPTH, if p == 0 { -10. } else { 10. }),
                    index_offset: Vec3::new(-CARD_WIDTH * 1.4, 0., 0.),
                    inverted_indexes: false,
                    keep_base_vertical: true,
                },
            );
            positions.insert(
                (CardOwners::Player(p), Stacks::Bases),
                PositionGenerator {
                    root: Vec3::new(-24., CARD_DEPTH, if p == 0 { -17. } else { 17. }),
                    index_offset: Vec3::new(0.95, CARD_DEPTH, if p == 0 { 0.5 } else { -0.5 }),
                    inverted_indexes: false,
                    keep_base_vertical: false,
                },
            );
            positions.insert(
                (CardOwners::Player(p), Stacks::DiscardPile),
                PositionGenerator {
                    root: Vec3::new(-32., 0., if p == 0 { -15. } else { 15. }),
                    index_offset: Vec3::new(0., -CARD_DEPTH, 0.),
                    inverted_indexes: true,
                    keep_base_vertical: true,
                },
            );
        }

        Self { positions }
    }
}
