use std::{
    any::Any, 
    collections::HashSet
};

use nalgebra::Vector3;
use winit::event::VirtualKeyCode;

use crate::engine::ecs::{
    Entity, 
    components::{
        movement_speeds::MovementSpeeds, 
        transform::Transform
    }, 
    world_coordinator::WorldCoordinator, 
    system::SystemTrait
};


#[derive(Default)]
pub struct UserControllerSystem {
    pub entities: HashSet<Entity>,
}

impl UserControllerSystem {
    pub fn new() -> Self {
        Self {
            entities: HashSet::new(),
        }
    }
}

impl SystemTrait for UserControllerSystem {
    fn entities(&self) -> &HashSet<Entity> {
        &self.entities
    }

    fn entities_mut(&mut self) -> &mut HashSet<Entity> {
        &mut self.entities
    }

    fn update(&self, entity: Entity, coordinator: &mut WorldCoordinator, dt: f32) {
        let translation = {
            let speeds = coordinator.get_component::<MovementSpeeds>(entity);
            speeds.translation
        };

        let direction = {
            let x = if coordinator.events().key_pressed(VirtualKeyCode::Left) {
                -1.0
            } else if coordinator.events().key_pressed(VirtualKeyCode::Right) { 
                1.0 
            } else {
                0.0
            };

            let y = if coordinator.events().key_pressed(VirtualKeyCode::Down) {
                -1.0
            } else if coordinator.events().key_pressed(VirtualKeyCode::Up) { 
                1.0 
            } else {
                0.0
            };

            Vector3::new(x, y, 0.0)
        };

        {
            let transform = coordinator.get_component_mut::<Transform>(entity);
            let delta = direction.component_mul(&translation) * dt;
            
            if delta == Vector3::zeros() {
                return;
            }

            transform.position += direction.component_mul(&translation) * dt;

            transform.dirty = true;
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}