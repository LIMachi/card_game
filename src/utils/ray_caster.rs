use crate::cards::{CARD_DEPTH, CARD_HEIGHT, CARD_WIDTH};
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;

#[derive(Component, Copy, Clone)]
pub struct OnMouseClick(MouseButton, SystemId<Entity, bool>);

pub fn click_handler(world: &mut World) {
    if world
        .resource::<ButtonInput<MouseButton>>()
        .get_just_pressed()
        .count()
        != 0
    {
        let hits: Vec<Entity> = world
            .query::<&RayCaster>()
            .iter(world)
            .filter_map(|c| {
                if let Some(hit) = c.hit {
                    Some(hit.entity)
                } else {
                    None
                }
            })
            .collect();
        let pressed: HashSet<MouseButton> = world
            .resource::<ButtonInput<MouseButton>>()
            .get_just_pressed()
            .copied()
            .collect();
        let mut run: Vec<(Entity, SystemId<Entity, bool>)> = Vec::new();
        let mut q = world.query::<&OnMouseClick>();
        for hit in &hits {
            if let Ok(click) = q.get(world, *hit) {
                if pressed.contains(&click.0) {
                    run.push((*hit, click.1.clone()));
                }
            }
        }
        for (entity, click) in &run {
            if world
                .run_system_with_input(*click, *entity)
                .is_ok_and(|r| r)
            {
                break;
            }
        }
    }
}

#[derive(Reflect, Debug, PartialEq, Copy, Clone)]
pub struct RayCastHit {
    pub entity: Entity,
    pub toi: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub relative: Vec3,
    pub back: bool,
    pub percent: Vec2,
    pub action: Option<u8>,
}

#[derive(Component, Reflect, Default, Debug, Copy, Clone)]
#[reflect(Component)]
pub struct RayCaster {
    pub hit: Option<RayCastHit>,
}

pub fn update_ray_cast(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut casters: Query<(&GlobalTransform, &Camera, &mut RayCaster), With<Camera3d>>,
    rapier_context: Res<RapierContext>,
    colliders: Query<&GlobalTransform, (Without<RayCaster>, With<Collider>)>,
) {
    if let Some(cursor) = windows.get_single().ok().and_then(|w| w.cursor_position()) {
        for (transform, camera, mut caster) in casters.iter_mut() {
            if let Some(ray) = camera.viewport_to_world(transform, cursor) {
                if let Some((
                    entity,
                    RayIntersection {
                        toi, point, normal, ..
                    },
                )) = rapier_context.cast_ray_and_get_normal(
                    ray.origin,
                    ray.direction.xyz(),
                    100.,
                    false,
                    QueryFilter::only_fixed(),
                ) {
                    let collider = colliders.get(entity).unwrap();
                    let relative = collider.affine().inverse().transform_point(point);
                    let back = relative.y <= 0.;
                    let percent = if back {
                        Vec2::new(
                            (relative.x + CARD_WIDTH / 2.) / CARD_WIDTH,
                            (relative.z + CARD_HEIGHT / 2.) / CARD_HEIGHT,
                        )
                    } else {
                        Vec2::new(
                            (CARD_WIDTH / 2. - relative.x) / CARD_WIDTH,
                            (relative.z + CARD_HEIGHT / 2.) / CARD_HEIGHT,
                        )
                    };
                    caster.hit = Some(RayCastHit {
                        entity,
                        toi,
                        point,
                        normal,
                        relative,
                        back,
                        percent,
                        action: None,
                    });
                } else {
                    caster.hit = None;
                }
            }
        }
    }
}

pub struct RayCasterPlugin;

impl Plugin for RayCasterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RayCastHit>()
            .register_type::<RayCaster>()
            .add_systems(PreUpdate, update_ray_cast)
            .add_systems(Update, click_handler);
    }
}
