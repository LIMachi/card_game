use crate::game::routines::{RoutineManager, Routines};
use crate::game::GameStates;
use crate::prelude::{ActionSet, DespawnRecursiveExt, Entity};
use crate::ui::choice_ui::{spawn_choices, ChoiceButton, ChoiceButtonNone, ChoiceRoot};
use bevy::ecs::system::{Command, CommandQueue};
use bevy::prelude::{Commands, NextState, World};

pub fn activate_card(world: &mut World) {
    let mut finished = false;
    if let Some(Routines::ActivateCard {
        card,
        owner,
        index,
        set,
        running,
    }) = world.resource::<RoutineManager>().routine()
    {
        if let Ok((root, root_state)) = world.query::<(Entity, &ChoiceRoot)>().get_single(world) {
            if root_state.finished {
                let mut choices = Vec::new();
                //extract vec of choices
                if let Ok(ChoiceButtonNone { selected: true }) =
                    world.query::<&ChoiceButtonNone>().get_single(world)
                {
                    //dirty trick with if let expansion (note: could also be done with a match trick)
                } else {
                    for (ChoiceButton { selected, index }) in
                        world.query::<&ChoiceButton>().iter(world)
                    {
                        if *selected {
                            choices.push(*index);
                        }
                    }
                }
                //despawn choice root
                let mut command_queue = CommandQueue::default();
                let mut commands = Commands::new(&mut command_queue, world);
                commands.entity(root).despawn_recursive();
                command_queue.apply(world);
                //apply choices as immediate actions
                if !choices.is_empty() {
                    match &set {
                        ActionSet::Optional(action) => {
                            world
                                .resource_mut::<RoutineManager>()
                                .action(owner, card, index, 0, *action);
                        }
                        ActionSet::OneAndOptional(_, action) => {
                            world
                                .resource_mut::<RoutineManager>()
                                .action(owner, card, index, 1, *action);
                        }
                        ActionSet::AnyOf2(first, second) => {
                            for index in &choices {
                                if *index == 0 {
                                    world
                                        .resource_mut::<RoutineManager>()
                                        .action(owner, card, *index, 0, *first);
                                } else if *index == 1 {
                                    world
                                        .resource_mut::<RoutineManager>()
                                        .action(owner, card, *index, 1, *second);
                                }
                            }
                        }
                        ActionSet::OneOf2(first, second) => {
                            if choices[0] == 0 {
                                world
                                    .resource_mut::<RoutineManager>()
                                    .action(owner, card, index, 0, *first);
                            } else if choices[0] == 1 {
                                world
                                    .resource_mut::<RoutineManager>()
                                    .action(owner, card, index, 1, *second);
                            }
                        }
                        _ => {}
                    }
                }
                //remove wait for finish
                finished = true;
            } else {
                //wait for root to signal a finish
                return;
            }
        } else if !running {
            finished = true;
            match &set {
                ActionSet::None => {}
                ActionSet::One(action) => {
                    // action.execute(world, index, card);
                    world
                        .resource_mut::<RoutineManager>()
                        .action(owner, card, index, 0, *action);
                }
                ActionSet::Optional(_) => {
                    world
                        .resource_mut::<NextState<GameStates>>()
                        .set(GameStates::ChoiceInput);
                    let mut command_queue = CommandQueue::default();
                    let mut commands = Commands::new(&mut command_queue, world);
                    spawn_choices(&mut commands, set);
                    command_queue.apply(world);
                    finished = false;
                }
                ActionSet::OneAndOptional(action, _) => {
                    action.execute(world, index, card);
                    world
                        .resource_mut::<NextState<GameStates>>()
                        .set(GameStates::ChoiceInput);
                    let mut command_queue = CommandQueue::default();
                    let mut commands = Commands::new(&mut command_queue, world);
                    spawn_choices(&mut commands, set);
                    command_queue.apply(world);
                    finished = false;
                }
                ActionSet::Two(first, second) => {
                    world
                        .resource_mut::<RoutineManager>()
                        .action(owner, card, index, 0, *first);
                    world
                        .resource_mut::<RoutineManager>()
                        .action(owner, card, index, 1, *second);
                }
                ActionSet::Three(first, second, third) => {
                    world
                        .resource_mut::<RoutineManager>()
                        .action(owner, card, index, 0, *first);
                    world
                        .resource_mut::<RoutineManager>()
                        .action(owner, card, index, 1, *second);
                    world
                        .resource_mut::<RoutineManager>()
                        .action(owner, card, index, 2, *third);
                }
                ActionSet::Four(first, second, third, fourth) => {
                    world
                        .resource_mut::<RoutineManager>()
                        .action(owner, card, index, 0, *first);
                    world
                        .resource_mut::<RoutineManager>()
                        .action(owner, card, index, 1, *second);
                    world
                        .resource_mut::<RoutineManager>()
                        .action(owner, card, index, 2, *third);
                    world
                        .resource_mut::<RoutineManager>()
                        .action(owner, card, index, 3, *fourth);
                }
                ActionSet::AnyOf2(_, _) => {
                    world
                        .resource_mut::<NextState<GameStates>>()
                        .set(GameStates::ChoiceInput);
                    let mut command_queue = CommandQueue::default();
                    let mut commands = Commands::new(&mut command_queue, world);
                    spawn_choices(&mut commands, set);
                    command_queue.apply(world);
                    finished = false;
                }
                ActionSet::OneOf2(_, _) => {
                    world
                        .resource_mut::<NextState<GameStates>>()
                        .set(GameStates::ChoiceInput);
                    let mut command_queue = CommandQueue::default();
                    let mut commands = Commands::new(&mut command_queue, world);
                    spawn_choices(&mut commands, set);
                    command_queue.apply(world);
                    finished = false;
                }
            }
        } else {
            //root is missing, abort
            //FIXME: should be an error
            finished = true;
        }
    }
    if finished {
        world.resource_mut::<RoutineManager>().finish();
    } else if let Some(Routines::ActivateCard {
        card,
        owner,
        index,
        set,
        running,
    }) = world.resource_mut::<RoutineManager>().routine_mut()
    {
        *running = true;
    }
}
