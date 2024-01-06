use crate::prelude::Component;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::EntityWorldMut;

pub trait FilterEnumInserter {
    fn insert(&self, entity: &mut EntityCommands);
    fn remove(&self, entity: &mut EntityCommands);
    fn insert_world(&self, entity: &mut EntityWorldMut);
    fn remove_world(&self, entity: &mut EntityWorldMut);
}
