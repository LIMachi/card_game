use crate::cards::actions::{ActionSet, OnPlay, OnScrap};
use crate::cards::assets::{Card, LoadedModels, LoadedSet};
use crate::cards::components::kinds::CardKinds;
use crate::cards::components::{CardCost, CardVisibility};
use crate::utils::filter_enum::FilterEnumInserter;
use bevy::prelude::*;

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
            ec.insert((CardCost(card.cost), CardVisibility::Hidden));
            card.kind.insert(&mut ec);
            for faction in &card.factions {
                faction.insert(&mut ec);
            }
            if card.play != ActionSet::None {
                ec.insert(OnPlay(card.play.clone()));
            }
            if card.scrap != ActionSet::None {
                ec.insert(OnScrap(card.scrap.clone()));
            }
            //FIXME: missing combo actions
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
