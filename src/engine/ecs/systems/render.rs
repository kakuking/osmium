use std::{
    any::Any, 
    collections::HashSet
};

use crate::engine::ecs::{
    Entity, 
    coordinator::WorldCoordinator, 
    system::SystemTrait
};


#[derive(Default)]
pub struct RenderSystem {
    pub entities: HashSet<Entity>,
}

impl RenderSystem {
    pub fn new() -> Self {
        Self {
            entities: HashSet::new(),
        }
    }
}

impl SystemTrait for RenderSystem {
    fn entities(&self) -> &HashSet<Entity> {
        &self.entities
    }

    fn entities_mut(&mut self) -> &mut HashSet<Entity> {
        &mut self.entities
    }

    fn update(&self, _entity: Entity, _coordinator: &mut WorldCoordinator, _dt: f32) { }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}