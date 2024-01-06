use crate::cards::prelude::*;
use crate::stacks::Stacks;
use crate::utils::filter_enum::FilterEnumInserter;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Reflect, Default, Debug, Copy, Clone)]
pub struct CardStateSnapshot {
    owner: CardOwners,
    stack: Stacks,
    index: CardIndex,
    visibility: CardVisibility,
}

#[derive(Component, Reflect, Default, Debug, Copy, Clone)]
#[reflect(Component)]
pub struct CardTransition {
    previous: CardStateSnapshot,
    previous_transform: Transform,
    next: CardStateSnapshot,
    next_transform: Transform,
    timer: f32,
    length: f32,
}

#[derive(Component, Reflect, Default, Debug, Copy, Clone)]
#[reflect(Component)]
pub struct StartTransition {
    pub owner: CardOwners,
    pub stack: Stacks,
    pub index: CardIndex,
    pub visibility: CardVisibility,
    pub length: f32,
}

// #[derive(Component, Reflect, Default, Debug, Copy, Clone)]
// #[reflect(Component)]
// pub struct FinishedTransitionCallback();

#[derive(Reflect, Default, Debug, Copy, Clone)]
pub struct PositionGenerator {
    pub root: Vec3,
    pub index_offset: Vec3,
    pub inverted_indexes: bool,
    pub keep_base_vertical: bool,
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct TransitionTransforms {
    pub positions: HashMap<(CardOwners, Stacks), PositionGenerator>,
}

impl TransitionTransforms {
    pub fn bake_transform(
        &self,
        snapshot: &CardStateSnapshot,
        stacks: &Query<&Stacks>,
        card_transform: &Transform,
        card_kind: &CardKinds,
    ) -> Transform {
        let mut out = Transform::from_rotation(Quat::from_axis_angle(
            Vec3::Z,
            if snapshot.visibility == CardVisibility::Visible {
                180f32.to_radians()
            } else {
                0.
            },
        ));
        if let Some(gen) = self.positions.get(&(snapshot.owner, snapshot.stack)) {
            let mut index = snapshot.index.0 as f32;
            if gen.inverted_indexes {
                let mut len = 0;
                for stack in stacks.iter() {
                    if *stack == snapshot.stack {
                        len += 1;
                    }
                }
                index = len as f32 - index - 1.;
            }
            out.translation = gen.root + gen.index_offset * index;
            if snapshot.visibility == CardVisibility::Visible
                && *card_kind != CardKinds::Ship
                && !gen.keep_base_vertical
                && card_transform.local_z().z >= 0.5
            //up vector is at least ~half way up (aka vertical card orientation)
            {
                out.rotate(Quat::from_axis_angle(Vec3::Y, 90f32.to_radians()));
            }
        }
        out
    }
}

pub fn start_transition(
    mut commands: Commands,
    transforms: Res<TransitionTransforms>,
    transitions: Query<(
        Entity,
        &StartTransition,
        &CardOwners,
        &Stacks,
        &CardIndex,
        &CardVisibility,
        &Transform,
        &CardKinds,
    )>,
    stacks: Query<&Stacks>,
) {
    for (card, &transition, &owner, &stack, &index, &visibility, transform, kind) in
        transitions.iter()
    {
        let mut ec = commands.entity(card);
        let next = CardStateSnapshot {
            owner: transition.owner,
            stack: transition.stack,
            index: transition.index,
            visibility: transition.visibility,
        };
        let next_transform = transforms.bake_transform(&next, &stacks, transform, kind);
        owner.remove(&mut ec);
        stack.remove(&mut ec);
        ec.remove::<(CardIndex, CardVisibility, StartTransition)>();
        ec.insert(CardTransition {
            previous: CardStateSnapshot {
                owner,
                stack,
                index,
                visibility,
            },
            previous_transform: *transform,
            next,
            next_transform,
            timer: 0.0,
            length: transition.length,
        });
    }
}

pub fn mirror_resource_change(
    mut commands: Commands,
    transforms: Res<TransitionTransforms>,
    stacks: Query<&Stacks>,
    still: Query<
        (
            Entity,
            &CardOwners,
            &Stacks,
            &CardIndex,
            &CardVisibility,
            &Transform,
            &CardKinds,
        ),
        Without<CardTransition>,
    >,
    mut transitioning: Query<(&mut CardTransition, &Transform, &CardKinds)>,
) {
    if transforms.is_changed() {
        for (card, &owner, &stack, &index, &visibility, transform, kind) in still.iter() {
            let mut ec = commands.entity(card);
            let next = CardStateSnapshot {
                owner,
                stack,
                index,
                visibility,
            };
            let next_transform = transforms.bake_transform(&next, &stacks, transform, kind);
            owner.remove(&mut ec);
            stack.remove(&mut ec);
            ec.remove::<(CardIndex, CardVisibility, StartTransition)>();
            ec.insert(CardTransition {
                previous: CardStateSnapshot {
                    owner,
                    stack,
                    index,
                    visibility,
                },
                previous_transform: *transform,
                next,
                next_transform,
                timer: 0.0,
                length: 0.0,
            });
        }
        for (mut transition, transform, kind) in transitioning.iter_mut() {
            transition.next_transform =
                transforms.bake_transform(&transition.next, &stacks, transform, kind);
        }
    }
}

pub fn update_transition(
    mut commands: Commands,
    mut transitions: Query<(Entity, &mut CardTransition, &mut Transform)>,
    time: Res<Time>,
) {
    for (card, mut transition, mut transform) in transitions.iter_mut() {
        transition.timer = (transition.timer + time.delta_seconds()).min(transition.length);
        let factor = if transition.length != 0. {
            transition.timer / transition.length
        } else {
            1.
        };
        transform.rotation = transition
            .previous_transform
            .rotation
            .slerp(transition.next_transform.rotation, factor);
        transform.translation = transition
            .previous_transform
            .translation
            .lerp(transition.next_transform.translation, factor);
        if transition.timer == transition.length {
            let mut card = commands.entity(card);
            card.remove::<CardTransition>();
            card.insert((transition.next.index, transition.next.visibility));
            transition.next.owner.insert(&mut card);
            transition.next.stack.insert(&mut card);
        }
    }
}

pub struct TransitionsPlugin;

impl Plugin for TransitionsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CardStateSnapshot>()
            .register_type::<CardTransition>()
            .register_type::<StartTransition>()
            .register_type::<PositionGenerator>()
            .register_type::<TransitionTransforms>()
            .init_resource::<TransitionTransforms>()
            .add_systems(PostUpdate, (start_transition, mirror_resource_change))
            .add_systems(PreUpdate, update_transition);
    }
}
