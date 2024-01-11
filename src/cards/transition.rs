use crate::cards::components::Focused;
use crate::cards::prelude::*;
use crate::stacks::Stacks;
use crate::utils::filter_enum::FilterEnumInserter;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource, Reflect, Default, Debug, Copy, Clone)]
#[reflect(Resource)]
pub struct PlayBackSpeed(pub f32);

#[derive(Reflect, Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct CardStateSnapshot {
    pub owner: CardOwners,
    pub stack: Stacks,
    pub index: CardIndex,
    pub visibility: CardVisibility,
}

#[derive(Component, Reflect, Default, Debug, Copy, Clone)]
#[reflect(Component)]
pub struct CardTransition {
    pub previous: CardStateSnapshot,
    pub previous_transform: Transform,
    pub next: CardStateSnapshot,
    pub next_transform: Transform,
    pub timer: f32,
    pub length: f32,
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

#[derive(Component, Reflect, Default, Debug, Copy, Clone)]
#[reflect(Component)]
pub struct StartFocus {
    pub length: f32,
}

#[derive(Component, Reflect, Default, Debug, Copy, Clone)]
#[reflect(Component)]
pub struct ResetFocus {
    pub length: f32,
}

// #[derive(Component, Reflect, Default, Debug, Copy, Clone)]
// #[reflect(Component)]
// pub struct FinishedTransitionCallback();

#[derive(Reflect, Default, Debug, Copy, Clone)]
pub struct PositionGenerator {
    pub root: Vec3,
    pub index_offset: Vec3,
    pub scale: Vec3,
    pub inverted_indexes: bool,
    pub keep_base_vertical: bool,
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct TransitionTransforms {
    pub positions: HashMap<(CardOwners, Stacks), PositionGenerator>,
}

#[derive(SystemSet, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TransitionSystemSets {
    Update,
    Start,
    Debug,
}

impl TransitionTransforms {
    pub fn bake_transform(
        &self,
        snapshot: &CardStateSnapshot,
        stacks: &Query<&Stacks>,
        card_kind: &CardKinds,
        focused: bool,
    ) -> Transform {
        let mut out = Transform::from_rotation(Quat::from_axis_angle(
            Vec3::Z,
            if snapshot.visibility == CardVisibility::Visible {
                180f32.to_radians()
            } else {
                0.
            },
        ));
        if let Some(gen) = if !focused {
            self.positions.get(&(snapshot.owner, snapshot.stack))
        } else {
            self.positions.get(&(CardOwners::Market, Stacks::Focused))
        } {
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
            out.scale = gen.scale;
            out.translation = gen.root + gen.index_offset * gen.scale * index;
            if snapshot.visibility == CardVisibility::Visible
                && *card_kind != CardKinds::Ship
                && !gen.keep_base_vertical
            {
                out.rotate(Quat::from_axis_angle(Vec3::Y, 90f32.to_radians()));
            }
        }
        out
    }
}

pub fn start_focus(
    mut commands: Commands,
    transforms: Res<TransitionTransforms>,
    transitions: Query<(
        Entity,
        &StartFocus,
        &CardOwners,
        &Stacks,
        &CardIndex,
        &CardVisibility,
        &Transform,
        &CardKinds,
    )>,
    stacks: Query<&Stacks>,
    speed: Res<PlayBackSpeed>,
) {
    for (card, &transition, &owner, &stack, &index, &visibility, transform, kind) in
        transitions.iter()
    {
        let mut ec = commands.entity(card);
        let next = CardStateSnapshot {
            owner,
            stack,
            index,
            visibility,
        };
        let next_transform = transforms.bake_transform(&next, &stacks, kind, true);
        ec.insert((
            Focused,
            CardTransition {
                previous: next,
                previous_transform: *transform,
                next,
                next_transform,
                timer: 0.0,
                length: transition.length * speed.0,
            },
        ))
        .remove::<StartFocus>();
    }
}

pub fn reset_focus(
    mut commands: Commands,
    transforms: Res<TransitionTransforms>,
    transitions: Query<
        (
            Entity,
            &ResetFocus,
            &CardOwners,
            &Stacks,
            &CardIndex,
            &CardVisibility,
            &Transform,
            &CardKinds,
        ),
        With<ResetFocus>,
    >,
    stacks: Query<&Stacks>,
    speed: Res<PlayBackSpeed>,
) {
    for (card, &transition, &owner, &stack, &index, &visibility, transform, kind) in
        transitions.iter()
    {
        let mut ec = commands.entity(card);
        let next = CardStateSnapshot {
            owner,
            stack,
            index,
            visibility,
        };
        let next_transform = transforms.bake_transform(&next, &stacks, kind, false);
        ec.insert((CardTransition {
            previous: next,
            previous_transform: *transform,
            next,
            next_transform,
            timer: 0.0,
            length: transition.length * speed.0,
        },))
            .remove::<(ResetFocus, Focused)>();
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
    speed: Res<PlayBackSpeed>,
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
        let next_transform = transforms.bake_transform(&next, &stacks, kind, false);
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
            length: transition.length * speed.0,
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
            Option<&Focused>,
        ),
        Without<CardTransition>,
    >,
    mut transitioning: Query<(
        &mut CardTransition,
        &Transform,
        &CardKinds,
        Option<&Focused>,
    )>,
) {
    if transforms.is_changed() {
        for (card, &owner, &stack, &index, &visibility, transform, kind, focused) in still.iter() {
            let mut ec = commands.entity(card);
            let next = CardStateSnapshot {
                owner,
                stack,
                index,
                visibility,
            };
            let next_transform = transforms.bake_transform(&next, &stacks, kind, focused.is_some());
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
        for (mut transition, transform, kind, focused) in transitioning.iter_mut() {
            transition.next_transform =
                transforms.bake_transform(&transition.next, &stacks, kind, focused.is_some());
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
        transform.scale = transition
            .previous_transform
            .scale
            .lerp(transition.next_transform.scale, factor);
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
            .register_type::<StartFocus>()
            .register_type::<ResetFocus>()
            .register_type::<PositionGenerator>()
            .register_type::<TransitionTransforms>()
            .init_resource::<TransitionTransforms>()
            .register_type::<PlayBackSpeed>()
            .insert_resource(PlayBackSpeed(0.5))
            .add_systems(
                PostUpdate,
                (
                    (start_transition, start_focus, reset_focus)
                        .in_set(TransitionSystemSets::Start),
                    mirror_resource_change.in_set(TransitionSystemSets::Debug),
                ),
            )
            .add_systems(
                PreUpdate,
                update_transition.in_set(TransitionSystemSets::Update),
            );
    }
}
