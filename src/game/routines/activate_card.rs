use crate::game::routines::{RoutineManager, Routines};
use crate::prelude::ActionSet;
use bevy::prelude::World;

pub fn activate_card(world: &mut World) {
    let mut finished = false;
    if let Some(Routines::ActivateCard { card, set, running }) =
        world.resource::<RoutineManager>().routine()
    {
        if !running {
            finished = true;
            match set {
                ActionSet::None => {}
                ActionSet::One(action) => {
                    action.execute(world, card);
                }
                ActionSet::Optional(option) => {
                    //check for auto
                    //spawn the options
                    // finished = false;
                }
                ActionSet::OneAndOptional(action, option) => {
                    action.execute(world, card);
                    //check for auto
                    //spawn the options
                    // finished = false;
                }
                ActionSet::All(v) => {
                    for action in &v {
                        action.execute(world, card);
                    }
                }
                ActionSet::Any(v) => {
                    //check for auto
                    //spawn the options
                    // finished = false;
                }
                ActionSet::OneOf(first, second) => {
                    //check for auto
                    //spawn the options
                    // finished = false;
                }
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
