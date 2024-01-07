use crate::cards::{CARD_DEPTH, CARD_HEIGHT, CARD_WIDTH};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;

#[derive(Reflect, Debug, PartialEq, Copy, Clone)]
pub struct RayCastHit {
    pub entity: Entity,
    pub toi: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub relative: Vec3,
    pub back: bool,
    pub percent: Vec2,
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
                    ray.direction,
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
            .add_systems(PreUpdate, update_ray_cast);
    }
}
