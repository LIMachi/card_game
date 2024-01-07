use crate::game::events::{GameEvent, GameEvents};
use crate::players::{LocalPlayer, Player, PlayerAttack, PlayerCounter, PlayerEconomy, PlayerLife};
use crate::prelude::*;
use crate::prelude::{
    AlignItems, BackgroundColor, BorderColor, ButtonBundle, Color, Commands, Component,
    FlexDirection, JustifyContent, NodeBundle, Reflect, Style, TextStyle, UiRect, Val,
};
use crate::states::turn::TurnStates;
use bevy_rapier3d::prelude::DebugRenderContext;

#[derive(Component, Debug, Reflect)]
pub struct LifeCounterUI;

#[derive(Component, Debug, Reflect)]
pub struct EconomyCounterUI;

#[derive(Component, Debug, Reflect)]
pub struct AttackCounterUI;

#[derive(Component, Debug, Reflect)]
pub struct PassTurnButton;

#[derive(Component, Debug, Reflect)]
pub struct AttackButton {
    player: u8,
}

pub fn spawn_ui(mut commands: Commands) {
    let life_style = TextStyle {
        font_size: 25.,
        color: Color::GREEN,
        ..Default::default()
    };
    let eco_style = TextStyle {
        font_size: 25.,
        color: Color::YELLOW,
        ..Default::default()
    };
    let atk_style = TextStyle {
        font_size: 25.,
        color: Color::RED,
        ..Default::default()
    };
    let enemy_style = Style {
        left: Val::Percent(89.5),
        top: Val::Percent(25.5),
        ..Default::default()
    };
    let ally_style = Style {
        left: Val::Percent(89.5),
        top: Val::Percent(51.),
        ..Default::default()
    };
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Default,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|root| {
            root.spawn((
                AttackButton { player: 1 },
                ButtonBundle {
                    style: Style {
                        width: Val::Px(160.),
                        height: Val::Px(45.),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..enemy_style.clone()
                    },
                    border_color: BorderColor(Color::RED),
                    background_color: BackgroundColor(Color::rgba(1., 0.7, 0.7, 1.)),
                    ..Default::default()
                },
            ))
            .with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "Attack",
                    TextStyle {
                        font_size: 30.,
                        color: Color::RED,
                        ..Default::default()
                    },
                ));
            });
            CardOwners::Player(1).insert(
                &mut root.spawn((
                    LifeCounterUI,
                    TextBundle::from_sections(vec![
                        TextSection {
                            value: "Life: ".to_string(),
                            style: life_style.clone(),
                        },
                        TextSection {
                            value: "0".to_string(),
                            style: life_style.clone(),
                        },
                    ])
                    .with_style(enemy_style.clone()),
                )),
            );
            CardOwners::Player(1).insert(
                &mut root.spawn((
                    EconomyCounterUI,
                    TextBundle::from_sections(vec![
                        TextSection {
                            value: "Economy: ".to_string(),
                            style: eco_style.clone(),
                        },
                        TextSection {
                            value: "0".to_string(),
                            style: eco_style.clone(),
                        },
                    ])
                    .with_style(enemy_style.clone()),
                )),
            );
            CardOwners::Player(1).insert(
                &mut root.spawn((
                    AttackCounterUI,
                    TextBundle::from_sections(vec![
                        TextSection {
                            value: "Attack: ".to_string(),
                            style: atk_style.clone(),
                        },
                        TextSection {
                            value: "0".to_string(),
                            style: atk_style.clone(),
                        },
                    ])
                    .with_style(enemy_style),
                )),
            );
            root.spawn((
                PassTurnButton,
                ButtonBundle {
                    style: Style {
                        width: Val::Px(160.),
                        height: Val::Px(45.),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..ally_style.clone()
                    },
                    border_color: BorderColor(Color::MIDNIGHT_BLUE),
                    background_color: BackgroundColor(Color::rgba(0.7, 0.7, 1., 1.)),
                    ..Default::default()
                },
            ))
            .with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "Pass turn",
                    TextStyle {
                        font_size: 30.,
                        color: Color::BLUE,
                        ..Default::default()
                    },
                ));
            });
            CardOwners::Player(0).insert(
                &mut root.spawn((
                    LifeCounterUI,
                    TextBundle::from_sections(vec![
                        TextSection {
                            value: "Life: ".to_string(),
                            style: life_style.clone(),
                        },
                        TextSection {
                            value: "0".to_string(),
                            style: life_style,
                        },
                    ])
                    .with_style(ally_style.clone()),
                )),
            );
            CardOwners::Player(0).insert(
                &mut root.spawn((
                    EconomyCounterUI,
                    TextBundle::from_sections(vec![
                        TextSection {
                            value: "Economy: ".to_string(),
                            style: eco_style.clone(),
                        },
                        TextSection {
                            value: "0".to_string(),
                            style: eco_style,
                        },
                    ])
                    .with_style(ally_style.clone()),
                )),
            );
            CardOwners::Player(0).insert(
                &mut root.spawn((
                    AttackCounterUI,
                    TextBundle::from_sections(vec![
                        TextSection {
                            value: "Attack: ".to_string(),
                            style: atk_style.clone(),
                        },
                        TextSection {
                            value: "0".to_string(),
                            style: atk_style,
                        },
                    ])
                    .with_style(ally_style.clone()),
                )),
            );
        });
}

pub fn update_attack_button(
    mut button: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &AttackButton,
        ),
        Changed<Interaction>,
    >,
    mut event: ResMut<GameEvent>,
) {
    for (interaction, mut background, mut border, target) in button.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                event.push(GameEvents::Attack {
                    as_much_as_possible: true, //FIXME: should replace this ui with actual buttons that react to other buttons
                    player: target.player,
                    base_index: None,
                });
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn update_pass_turn_button(
    mut button: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<PassTurnButton>),
    >,
    turn: Res<State<TurnStates>>,
    debug: Res<DebugRenderContext>,
    mut event: ResMut<GameEvent>,
    local_player: Res<LocalPlayer>,
) {
    if *turn.get() == TurnStates::PlayerTurn(local_player.0) || debug.enabled {
        for (interaction, mut background, mut border) in button.iter_mut() {
            match interaction {
                Interaction::Pressed => {
                    *background = Color::rgba(0.4, 0.4, 1., 1.).into();
                    *border = Color::rgba(0.1, 0.1, 0.3, 1.).into();
                    event.push(GameEvents::PassTurn);
                }
                Interaction::Hovered => {
                    *background = Color::rgba(0.6, 0.6, 1., 1.).into();
                    *border = Color::rgba(0.2, 0.2, 0.6, 1.).into();
                }
                Interaction::None => {
                    *background = Color::rgba(0.7, 0.7, 1., 1.).into();
                    *border = Color::MIDNIGHT_BLUE.into();
                }
            }
        }
    } else {
        for (interaction, mut background, mut border) in button.iter_mut() {
            match interaction {
                Interaction::None => {
                    *background = Color::rgba(0.7, 0.7, 1., 1.).into();
                    *border = Color::MIDNIGHT_BLUE.into();
                }
                _ => {
                    *background = Color::MAROON.into();
                    *border = Color::RED.into();
                }
            }
        }
    }
}

pub fn update_counter_ui<const PLAYER: u8, C: Component + PlayerCounter, T: Component>(
    mut ui: Query<&mut Text, (With<Player<PLAYER>>, With<T>)>,
    counter: Query<&C, (Changed<C>, With<Player<PLAYER>>)>,
) {
    if let Ok(mut text) = ui.get_single_mut() {
        if let Ok(value) = counter.get_single() {
            text.sections[1].value = value.get_value().to_string();
        }
    }
}

pub struct CountersUIPlugin;

impl Plugin for CountersUIPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LifeCounterUI>()
            .register_type::<EconomyCounterUI>()
            .register_type::<AttackCounterUI>()
            .register_type::<AttackButton>()
            .register_type::<PassTurnButton>()
            .add_systems(Startup, spawn_ui)
            .add_systems(Update, (update_pass_turn_button, update_attack_button))
            .add_systems(
                PostUpdate,
                (
                    update_counter_ui::<0, PlayerLife, LifeCounterUI>,
                    update_counter_ui::<0, PlayerAttack, AttackCounterUI>,
                    update_counter_ui::<0, PlayerEconomy, EconomyCounterUI>,
                    update_counter_ui::<1, PlayerLife, LifeCounterUI>,
                    update_counter_ui::<1, PlayerAttack, AttackCounterUI>,
                    update_counter_ui::<1, PlayerEconomy, EconomyCounterUI>,
                ),
            );
    }
}
