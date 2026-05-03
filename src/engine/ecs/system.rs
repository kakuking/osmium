use std::{any::Any, collections::HashSet};

use crate::engine::ecs::{
    Entity, 
    coordinator::WorldCoordinator
};

pub trait SystemTrait {
    fn entities(&self) -> &HashSet<Entity>;
    fn entities_mut(&mut self) -> &mut HashSet<Entity>;

    fn update(&self, entity: Entity, coordinator: &mut WorldCoordinator, dt: f32);
    fn update_all_entities(&self, coordinator: &mut WorldCoordinator, dt: f32) {
        for entity in self.entities().iter().copied() {
            self.update(entity, coordinator, dt);
        }
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
