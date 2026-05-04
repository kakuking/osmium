use std::{any::Any, collections::HashSet};

use crate::engine::ecs::{
    Entity, 
    world_coordinator::WorldCoordinator, 
};

pub trait SystemTrait {
    fn entities(&self) -> &HashSet<Entity>;
    fn entities_mut(&mut self) -> &mut HashSet<Entity>;

    fn initialize(&self, _world: &mut WorldCoordinator) {}

    fn update(&self, entity: Entity, world: &mut WorldCoordinator, dt: f32);
    fn update_all_entities(&self, world: &mut WorldCoordinator, dt: f32) {
        for entity in self.entities().iter().copied() {
            self.update(entity, world, dt);
        }
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
