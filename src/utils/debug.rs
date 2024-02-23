use crate::players::Player;
use crate::prelude::{CardIndex, CardOwners, CardVisibility, MarketOwned, Stacks, StartTransition};
use crate::stacks::Hand;
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::{DebugRenderContext, RapierDebugRenderPlugin};

pub fn toggle_debug(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut debug: ResMut<DebugRenderContext>,
    mut root: Query<&mut Visibility, With<FpsRoot>>,
    enemy_hand: Query<
        (Entity, &CardOwners, &CardIndex),
        (With<Hand>, Without<MarketOwned>, Without<Player<0>>),
    >,
) {
    if keys.just_pressed(KeyCode::ContextMenu) {
        debug.enabled ^= true;
        let mut visibility = root.single_mut();
        *visibility = if debug.enabled {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        for (card, &owner, &index) in enemy_hand.iter() {
            commands.entity(card).insert(StartTransition {
                owner,
                stack: Stacks::Hand,
                index,
                visibility: if debug.enabled {
                    CardVisibility::Visible
                } else {
                    CardVisibility::Hidden
                },
                length: 0.0,
            });
        }
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct FpsRoot;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct FpsText;

pub fn setup_fps_counter(mut commands: Commands) {
    let root = commands
        .spawn((
            FpsRoot,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(1.),
                    top: Val::Percent(1.),
                    bottom: Val::Auto,
                    left: Val::Auto,
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    let text_fps = commands
        .spawn((
            FpsText,
            TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: "FPS: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[text_fps]);
}

pub fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            text.sections[1].value = format!("{value:>4.0}");
            text.sections[1].style.color = if value >= 120.0 {
                Color::rgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                Color::rgb((1.0 - (value - 60.0) / (120.0 - 60.0)) as f32, 1.0, 0.0)
            } else if value >= 30.0 {
                Color::rgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
            } else {
                Color::rgb(1.0, 0.0, 0.0)
            }
        } else {
            text.sections[1].value = " N/A".into();
            text.sections[1].style.color = Color::WHITE;
        }
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FpsRoot>()
            .register_type::<FpsText>()
            .add_plugins((
                RapierDebugRenderPlugin::default(),
                WorldInspectorPlugin::default().run_if(|d: Res<DebugRenderContext>| d.enabled),
                FrameTimeDiagnosticsPlugin::default(),
            ))
            .add_systems(Startup, setup_fps_counter)
            .add_systems(Update, (toggle_debug, fps_text_update_system));
    }
}
