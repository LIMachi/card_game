use crate::game::routines::{RoutineManager, Routines};
use crate::prelude::{ActionSet, DespawnRecursiveExt, Entity};
use crate::ui::choice_ui::{spawn_choices, ChoiceButton, ChoiceButtonNone, ChoiceRoot};
use bevy::ecs::system::{Command, CommandQueue};
use bevy::prelude::{Commands, World};

pub fn activate_card(world: &mut World) {
    let mut finished = false;
    if let Some(Routines::ActivateCard { card, set, running }) =
        world.resource::<RoutineManager>().routine()
    {
        if !running {
            finished = true;
            match &set {
                ActionSet::None => {}
                ActionSet::One(action) => {
                    action.execute(world, card);
                }
                ActionSet::Optional(_) => {
                    let mut command_queue = CommandQueue::default();
                    let mut commands = Commands::new(&mut command_queue, world);
                    spawn_choices(&mut commands, set.clone());
                    command_queue.apply(world);
                    finished = false;
                }
                ActionSet::OneAndOptional(action, _) => {
                    action.execute(world, card);
                    let mut command_queue = CommandQueue::default();
                    let mut commands = Commands::new(&mut command_queue, world);
                    spawn_choices(&mut commands, set.clone());
                    command_queue.apply(world);
                    finished = false;
                }
                ActionSet::All(v) => {
                    for action in v {
                        action.execute(world, card);
                    }
                }
                ActionSet::Any(_) => {
                    let mut command_queue = CommandQueue::default();
                    let mut commands = Commands::new(&mut command_queue, world);
                    spawn_choices(&mut commands, set.clone());
                    command_queue.apply(world);
                    finished = false;
                }
                ActionSet::OneOf(_, _) => {
                    let mut command_queue = CommandQueue::default();
                    let mut commands = Commands::new(&mut command_queue, world);
                    spawn_choices(&mut commands, set.clone());
                    command_queue.apply(world);
                    finished = false;
                }
            }
        } else {
            if let Ok((root, root_state)) = world.query::<(Entity, &ChoiceRoot)>().get_single(world)
            {
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
                                action.execute(world, card);
                            }
                            ActionSet::OneAndOptional(_, action) => {
                                action.execute(world, card);
                            }
                            ActionSet::Any(actions) => {
                                for index in &choices {
                                    actions[*index as usize].execute(world, card);
                                }
                            }
                            ActionSet::OneOf(first, second) => {
                                if choices[0] == 0 {
                                    first.execute(world, card);
                                } else {
                                    second.execute(world, card);
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
            } else {
                //root is missing, abort
                //FIXME: should be an error
                finished = true;
            }
        }
    }
    if finished {
        world.resource_mut::<RoutineManager>().finish();
    } else if let Some(Routines::ActivateCard { card, set, running }) =
        world.resource_mut::<RoutineManager>().routine_mut()
    {
        *running = true;
    }
}
