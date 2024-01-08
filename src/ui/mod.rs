use bevy::prelude::*;

pub mod choice_ui;
pub mod player_counters;

#[derive(Component, Debug, Reflect)]
pub struct UIRoot;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, |mut commands: Commands| {
            commands.spawn((
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
            ));
        })
        .add_plugins((player_counters::CountersUIPlugin, choice_ui::ChoiceUIPlugin));
    }
}
