use crate::ui::billboards::{
    attach_life_billboard_when_base_enter_play, remove_billboard_when_base_leave_play,
    update_life_billboard,
};
use bevy::prelude::*;

pub mod billboards;
pub mod choice_ui;
pub mod player_counters;

#[derive(Component, Debug, Reflect)]
pub struct UIRoot;

#[derive(Component, Debug, Reflect)]
pub struct SelectionValidationButton;

pub fn ui_setup(mut commands: Commands) {
    commands
        .spawn((
            UIRoot,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Default,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                SelectionValidationButton,
                ButtonBundle {
                    visibility: Visibility::Hidden,
                    background_color: BackgroundColor(Color::BLACK.with_a(0.9)),
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_self: JustifySelf::Center,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(5.)),
                        margin: UiRect::all(Val::Px(5.)),
                        border: UiRect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .with_children(|button| {
                button.spawn(TextBundle {
                    text: Text {
                        alignment: TextAlignment::Center,
                        ..Text::from_section(
                            "Validate selection",
                            TextStyle {
                                font_size: 20.,
                                ..Default::default()
                            },
                        )
                    },
                    ..Default::default()
                });
            });
        });
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, ui_setup)
            .add_plugins((player_counters::CountersUIPlugin, choice_ui::ChoiceUIPlugin))
            .add_systems(
                Update,
                (
                    attach_life_billboard_when_base_enter_play,
                    remove_billboard_when_base_leave_play,
                ),
            )
            .add_systems(PostUpdate, update_life_billboard);
    }
}
