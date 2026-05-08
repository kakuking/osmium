use std::{
    any::Any, 
    collections::HashSet
};

use nalgebra::{Unit, UnitQuaternion, Vector3};
use winit::event::VirtualKeyCode;

use crate::engine::ecs::{
    Entity, 
    components::{
        default_controller::DefaultController, 
        transform::Transform
    }, 
    world_coordinator::WorldCoordinator, 
    system::SystemTrait
};


#[derive(Default)]
pub struct DefaultControllerSystem {
    pub entities: HashSet<Entity>,
}

impl DefaultControllerSystem {
    pub fn new() -> Self {
        Self {
            entities: HashSet::new(),
        }
    }
}

impl SystemTrait for DefaultControllerSystem {
    fn entities(&self) -> &HashSet<Entity> {
        &self.entities
    }

    fn entities_mut(&mut self) -> &mut HashSet<Entity> {
        &mut self.entities
    }

    fn update(&self, entity: Entity, coordinator: &mut WorldCoordinator, dt: f32) {
        let translation = {
            let speeds = coordinator.get_component::<DefaultController>(entity);
            speeds.translation
        };

        let direction = {
            let x = if coordinator.events().key_pressed(VirtualKeyCode::A) {
                -1.0
            } else if coordinator.events().key_pressed(VirtualKeyCode::D) { 
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

            let z = if coordinator.events().key_pressed(VirtualKeyCode::S) {
                -1.0
            } else if coordinator.events().key_pressed(VirtualKeyCode::W) { 
                1.0 
            } else {
                0.0
            };

            Vector3::new(x, y, z)
        };

        let rotation_speed = 2.0;

        let yaw = if coordinator.events().key_pressed(VirtualKeyCode::Left) {
            rotation_speed * dt
        } else if coordinator.events().key_pressed(VirtualKeyCode::Right) {
            -rotation_speed * dt
        } else {
            0.0
        };

        {
            let transform = coordinator.get_component_mut::<Transform>(entity);
            let delta = direction.component_mul(&translation) * dt;
            
            if delta != Vector3::zeros() {
                transform.position += direction.component_mul(&translation) * dt;
                transform.dirty = true;
            }

            if yaw != 0.0 {
                let rotation = UnitQuaternion::from_axis_angle(
                    &Unit::new_normalize(Vector3::y()),
                    yaw,
                );

                transform.rotation = rotation * transform.rotation;
                transform.dirty = true;
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}