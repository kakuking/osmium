use std::{
    any::Any, 
    collections::HashSet
};

use crate::engine::ecs::{
    Entity, 
    components::{
        camera::Camera, 
        transform::Transform
    }, 
    system::SystemTrait, 
    world_coordinator::WorldCoordinator
};


#[derive(Default)]
pub struct CameraSystem {
    pub entities: HashSet<Entity>,
}

impl CameraSystem {
    pub fn new() -> Self {
        Self {
            entities: HashSet::new(),
        }
    }
}

impl SystemTrait for CameraSystem {
    fn entities(&self) -> &HashSet<Entity> {
        &self.entities
    }

    fn entities_mut(&mut self) -> &mut HashSet<Entity> {
        &mut self.entities
    }

    fn update(&self, entity: Entity, world: &mut WorldCoordinator, _dt: f32) {
        let transform_dirty = world.get_component_mut::<Transform>(entity).dirty;

        let camera = world.get_component_mut::<Camera>(entity);

        if transform_dirty {
            camera.dirty = true;
            let transform = world.get_component_mut::<Transform>(entity);
            transform.dirty = false;
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}