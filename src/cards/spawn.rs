use crate::cards::actions::CardActions;
use crate::cards::assets::{Card, LoadedModels, LoadedSet};
use crate::cards::prelude::*;
use crate::utils::filter_enum::FilterEnumInserter;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_rapier3d::prelude::{Collider, RigidBody};

#[derive(Component)]
pub struct SpawnCard(pub String);

pub fn spawn_card(
    mut commands: Commands,
    to_spawn: Query<(Entity, &SpawnCard)>,
    set: Res<LoadedSet>,
    models: Res<LoadedModels>,
    cards: Res<Assets<Card>>,
) {
    for (entity, spawn) in to_spawn.iter() {
        if let Some(card) = set.cards.get(&spawn.0).and_then(|h| cards.get(h)) {
            let mut ec = commands.entity(entity);
            ec.insert((
                CardCost(card.cost),
                CardVisibility::Hidden,
                Collider::cuboid(CARD_WIDTH / 2., CARD_DEPTH / 2., CARD_HEIGHT / 2.),
                RigidBody::Fixed,
            ));
            card.kind.insert(&mut ec);
            let mut factions = HashSet::with_capacity(card.factions.len());
            for faction in &card.factions {
                faction.insert(&mut ec);
                factions.insert(*faction);
            }
            ec.insert((
                CardFactions(factions),
                CardActions::from_serialized_card(card),
            ));
            ec.with_children(|parent| {
                parent.spawn(PbrBundle {
                    mesh: models.back_mesh.clone(),
                    material: models.back_material.clone(),
                    ..Default::default()
                });
                parent.spawn(PbrBundle {
                    mesh: models.side_mesh.clone(),
                    material: models.side_material.clone(),
                    ..Default::default()
                });
                if card.kind == CardKinds::Ship {
                    parent.spawn(PbrBundle {
                        mesh: models.front_ship_mesh.clone(),
                        material: models.front_materials.get(&spawn.0).unwrap().clone(),
                        ..Default::default()
                    });
                } else {
                    parent.spawn(PbrBundle {
                        mesh: models.front_base_mesh.clone(),
                        material: models.front_materials.get(&spawn.0).unwrap().clone(),
                        ..Default::default()
                    });
                }
            });
            ec.remove::<SpawnCard>();
        }
    }
}
