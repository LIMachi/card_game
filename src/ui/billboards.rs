use crate::cards::components::kinds::BaseLife;
use crate::prelude::*;
// use bevy_mod_billboard::BillboardTextBundle;
/*
#[derive(Component, Debug, Reflect)]
pub struct BaseLifeBillboard;

pub fn update_life_billboard(
    bases: Query<(&BaseLife, &Children, Option<&Base>, Option<&Outpost>), Changed<BaseLife>>,
    mut billboards: Query<&mut Text, With<BaseLifeBillboard>>,
) {
    for (life, children, base, outpost) in bases.iter() {
        for child in children.iter() {
            if let Ok(mut text) = billboards.get_mut(*child) {
                text.sections[0].value = format!("{}", life.0);
                if let Some(base) = base {
                    if base.0 == life.0 {
                        text.sections[0].style.color = Color::GREEN;
                    } else {
                        text.sections[0].style.color = Color::RED;
                    }
                }
                if let Some(outpost) = outpost {
                    if outpost.0 == life.0 {
                        text.sections[0].style.color = Color::GREEN;
                    } else {
                        text.sections[0].style.color = Color::RED;
                    }
                }
            }
        }
    }
}

pub fn attach_life_billboard_when_base_enter_play(
    mut commands: Commands,
    bases: Query<(Entity, &BaseLife), Added<Bases>>,
) {
    for (base, life) in bases.iter() {
        commands.entity(base).with_children(|base| {
            base.spawn((
                BaseLifeBillboard,
                BillboardTextBundle {
                    transform: Transform::from_xyz(-2.3, 0.2, 3.75)
                        .with_scale(Vec3::new(0.01, 1., 0.01)),
                    text: Text::from_section(
                        format!("{}", life.0),
                        TextStyle {
                            color: Color::GREEN,
                            font_size: 150.0,
                            ..Default::default()
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..Default::default()
                },
            ));
        });
    }
}

pub fn remove_billboard_when_base_leave_play(
    mut commands: Commands,
    bases: Query<(&Children, &CardTransition), (With<BaseLife>, Added<CardTransition>)>,
    billboards: Query<Entity, With<BaseLifeBillboard>>,
) {
    for (children, transition) in bases.iter() {
        if transition.next.stack != Stacks::Bases {
            for child in children.iter() {
                if billboards.contains(*child) {
                    commands.entity(*child).despawn_recursive();
                }
            }
        }
    }
}
*/
