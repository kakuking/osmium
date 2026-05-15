use std::{
    any::Any, 
    collections::HashSet
};

use crate::engine::ecs::{
    Entity, 
    components::{
        light::Light, 
        transform::Transform
    }, 
    system::SystemTrait, 
    world_coordinator::WorldCoordinator
};


#[derive(Default)]
pub struct LightSystem {
    pub entities: HashSet<Entity>,
}

impl LightSystem {
    pub fn new() -> Self {
        Self {
            entities: HashSet::new(),
        }
    }
}

impl SystemTrait for LightSystem {
    fn entities(&self) -> &HashSet<Entity> {
        &self.entities
    }

    fn entities_mut(&mut self) -> &mut HashSet<Entity> {
        &mut self.entities
    }

    fn initialize(&mut self, world: &mut WorldCoordinator) {
        for entity in self.entities().iter().copied() {
            let transform = *world.get_component::<Transform>(entity);

            let light = world.get_component_mut::<Light>(entity);

            light.dirty = true;

            light.get_light_data(&transform);
        }
    }

    fn update(&self, _entity: Entity, _coordinator: &mut WorldCoordinator, _dt: f32) { }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}