use crate::game::GameStates;
use crate::prelude::*;
use bevy::ecs::system::EntityCommands;

#[derive(Component, Debug, Reflect)]
pub struct ChoiceRoot {
    pub finished: bool,
}

#[derive(Component, Debug, Reflect)]
pub struct ChoiceButton {
    pub selected: bool,
    pub index: u8,
}

#[derive(Component, Debug, Reflect)]
pub struct ValidateButton;

#[derive(Component, Debug, Reflect)]
pub struct ChoiceButtonNone {
    pub selected: bool,
}

pub struct ChoiceUIPlugin;

impl Plugin for ChoiceUIPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChoiceRoot>()
            .register_type::<ChoiceButton>()
            .register_type::<ValidateButton>()
            .register_type::<ChoiceButtonNone>()
            .add_systems(
                Update,
                (handle_choice_clicks, handle_choice_hover)
                    .run_if(in_state(GameStates::ChoiceInput)),
            );
    }
}

pub fn handle_choice_hover(
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (
            Changed<Interaction>,
            Or<(
                With<ChoiceButton>,
                With<ValidateButton>,
                With<ChoiceButtonNone>,
            )>,
        ),
    >,
) {
    for (state, mut back, mut border) in buttons.iter_mut() {
        if *state == Interaction::None {
            *back = BackgroundColor(Color::BLACK.with_a(0.7));
        } else {
            *back = BackgroundColor(Color::GRAY);
        }
    }
}

pub fn handle_choice_clicks(
    mouse_buttons: Res<Input<MouseButton>>,
    mut root: Query<&mut ChoiceRoot>,
    validate_button: Query<&Interaction, With<ValidateButton>>,
    mut buttons: Query<
        (
            &mut ChoiceButton,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        Without<ChoiceButtonNone>,
    >,
    mut do_nothing: Query<(
        &mut ChoiceButtonNone,
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
    mut state: ResMut<NextState<GameStates>>,
) {
    if mouse_buttons.just_released(MouseButton::Left) {
        if let Ok(mut root) = root.get_single_mut() {
            let multi = if let Ok(validate) = validate_button.get_single() {
                if *validate != Interaction::None {
                    root.finished = true;
                }
                true
            } else {
                false
            };
            if root.finished {
                state.set(GameStates::MainLoop);
                return;
            }
            let mut hit = None;
            for (mut button, state, mut back, mut border) in buttons.iter_mut() {
                if *state != Interaction::None {
                    button.selected ^= true;
                    if button.selected {
                        hit = Some(button.index + 1);
                        *border = Color::CYAN.into();
                        break;
                    } else {
                        *border = Default::default();
                    }
                }
            }
            if let Ok((mut button, state, mut back, mut border)) = do_nothing.get_single_mut() {
                if *state != Interaction::None {
                    button.selected ^= true;
                    if button.selected {
                        hit = Some(0);
                        *border = Color::CYAN.into();
                    } else {
                        *border = Default::default();
                    }
                }
            }
            if let Some(hit) = hit {
                if !multi {
                    //the toggling off of other buttons is overkill, since it shouldn't be possible to have another button pressed due to the finish flag being set after each click
                    for (mut button, _, mut back, mut border) in buttons.iter_mut() {
                        if button.selected && button.index + 1 != hit {
                            button.selected = false;
                            *border = Default::default();
                        }
                    }
                    if hit != 0 {
                        if let Ok((mut button, _, mut back, mut border)) =
                            do_nothing.get_single_mut()
                        {
                            button.selected = false;
                            *border = Default::default();
                        }
                    }
                    state.set(GameStates::MainLoop);
                    root.finished = true;
                }
            }
        }
    }
}

pub fn spawn_choices(commands: &mut Commands, set: ActionSet) {
    fn spawn_ui<'w, 's, 'c>(commands: &'c mut Commands<'w, 's>) -> EntityCommands<'w, 's, 'c> {
        commands.spawn((
            ChoiceRoot { finished: false },
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_a(0.8)),
                style: Style {
                    // height: Val::Auto,
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    top: Val::Percent(14.),
                    padding: UiRect::all(Val::Px(5.)),
                    margin: UiRect::all(Val::Px(5.)),
                    border: UiRect::all(Val::Px(5.)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
    }
    let insert_button = |ec: &mut EntityCommands, content: String| {
        ec.insert(ButtonBundle {
            background_color: BackgroundColor(Color::BLACK.with_a(0.9)),
            style: Style {
                align_items: AlignItems::Center,
                width: Val::Percent(20.),
                height: Val::Percent(90.),
                justify_self: JustifySelf::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(5.)),
                margin: UiRect::all(Val::Px(5.)),
                border: UiRect::all(Val::Px(5.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|button| {
            button.spawn(TextBundle {
                text: Text {
                    alignment: TextAlignment::Center,
                    ..Text::from_section(
                        content,
                        TextStyle {
                            font_size: 20.,
                            ..Default::default()
                        },
                    )
                },
                ..Default::default()
            });
        });
    };
    let spawn_separator = |root: &mut ChildBuilder, content: &str| {
        root.spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_self: JustifySelf::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(5.)),
                margin: UiRect::all(Val::Px(5.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|separator| {
            separator.spawn(TextBundle {
                text: Text {
                    alignment: TextAlignment::Center,
                    ..Text::from_section(
                        content,
                        TextStyle {
                            font_size: 20.,
                            ..Default::default()
                        },
                    )
                },
                ..Default::default()
            });
        });
    };
    match set {
        ActionSet::Optional(option) => {
            let mut ec = spawn_ui(commands);
            ec.with_children(|root| {
                let mut button = root.spawn(ChoiceButton {
                    selected: false,
                    index: 0,
                });
                insert_button(&mut button, format!("{option}"));
                spawn_separator(root, "or");
                let mut button = root.spawn(ChoiceButtonNone { selected: false });
                insert_button(&mut button, "nothing".to_string());
            });
        }
        ActionSet::OneAndOptional(_, option) => {
            let mut ec = spawn_ui(commands);
            ec.with_children(|root| {
                let mut button = root.spawn(ChoiceButton {
                    selected: false,
                    index: 0,
                });
                insert_button(&mut button, format!("{option}"));
                spawn_separator(root, "or");
                let mut button = root.spawn(ChoiceButtonNone { selected: false });
                insert_button(&mut button, "nothing".to_string());
            });
        }
        ActionSet::AnyOf2(first, second) => {
            let mut ec = spawn_ui(commands);
            ec.with_children(|root| {
                let mut button = root.spawn(ChoiceButton {
                    selected: false,
                    index: 0,
                });
                insert_button(&mut button, format!("{}", first));
                spawn_separator(root, "and/or");
                let mut button = root.spawn(ChoiceButton {
                    selected: false,
                    index: 1,
                });
                insert_button(&mut button, format!("{}", second));
                spawn_separator(root, "OR");
                let mut button = root.spawn(ChoiceButtonNone { selected: false });
                insert_button(&mut button, "nothing".to_string());
                let mut button = root.spawn(ValidateButton);
                insert_button(&mut button, "Validate".to_string());
            });
        }
        ActionSet::OneOf2(first, second) => {
            let mut ec = spawn_ui(commands);
            ec.with_children(|root| {
                let mut button = root.spawn(ChoiceButton {
                    selected: false,
                    index: 0,
                });
                insert_button(&mut button, format!("{first}"));
                spawn_separator(root, "or");
                let mut button = root.spawn(ChoiceButton {
                    selected: false,
                    index: 1,
                });
                insert_button(&mut button, format!("{second}"));
            });
        }
        _ => {}
    }
}
