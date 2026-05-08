use std::{
    any::Any,
    collections::HashSet,
};

use nalgebra::{UnitQuaternion, Vector3};
use winit::event::VirtualKeyCode;

use crate::engine::ecs::{
    Entity,
    components::{
        camera::Camera,
        default_first_person_controller::FirstPersonController,
        transform::Transform,
    },
    world_coordinator::WorldCoordinator,
    system::SystemTrait,
};

#[derive(Default)]
pub struct FirstPersonControllerSystem {
    pub entities: HashSet<Entity>,
}

impl FirstPersonControllerSystem {
    pub fn new() -> Self {
        Self {
            entities: HashSet::new(),
        }
    }
}

impl SystemTrait for FirstPersonControllerSystem {
    fn entities(&self) -> &HashSet<Entity> {
        &self.entities
    }

    fn entities_mut(&mut self) -> &mut HashSet<Entity> {
        &mut self.entities
    }

    fn update(&self, entity: Entity, coordinator: &mut WorldCoordinator, dt: f32) {
        let (mouse_dx, mouse_dy) = coordinator.events().mouse_delta();

        {
            let controller = coordinator.get_component_mut::<FirstPersonController>(entity);

            controller.yaw -= mouse_dx as f32 * controller.mouse_sensitivity;
            controller.pitch -= mouse_dy as f32 * controller.mouse_sensitivity;

            controller.pitch = controller.pitch.clamp(
                -controller.pitch_limit,
                controller.pitch_limit,
            );
        }

        let (move_speed, yaw, pitch) = {
            let controller = coordinator.get_component::<FirstPersonController>(entity);

            (
                controller.move_speed,
                controller.yaw,
                controller.pitch,
            )
        };

        let direction = {
            let mut dir = Vector3::zeros();

            if coordinator.events().key_pressed(VirtualKeyCode::W) {
                dir.z -= 1.0;
            }

            if coordinator.events().key_pressed(VirtualKeyCode::S) {
                dir.z += 1.0;
            }

            if coordinator.events().key_pressed(VirtualKeyCode::A) {
                dir.x -= 1.0;
            }

            if coordinator.events().key_pressed(VirtualKeyCode::D) {
                dir.x += 1.0;
            }

            if coordinator.events().key_pressed(VirtualKeyCode::Space) {
                dir.y += 1.0;
            }

            if coordinator.events().key_pressed(VirtualKeyCode::LShift) {
                dir.y -= 1.0;
            }

            if dir.norm_squared() > 0.0 {
                dir.normalize()
            } else {
                dir
            }
        };

        let rotation =
            UnitQuaternion::from_euler_angles(0.0, yaw, 0.0) *
            UnitQuaternion::from_euler_angles(pitch, 0.0, 0.0);

        let movement = rotation * direction * move_speed * dt;

        let rotation =
            UnitQuaternion::from_euler_angles(0.0, yaw, 0.0)
            * UnitQuaternion::from_euler_angles(pitch, 0.0, 0.0);

        let changed =
            movement != Vector3::zeros()
            || mouse_dx != 0.0
            || mouse_dy != 0.0;

        {
            let transform = coordinator.get_component_mut::<Transform>(entity);

            if movement != Vector3::zeros() {
                transform.position += movement;
            }

            if mouse_dx != 0.0 || mouse_dy != 0.0 {
                transform.rotation = rotation;
            }

            if changed {
                transform.dirty = true;
            }
        }

        {
            let camera = coordinator.get_component_mut::<Camera>(entity);

            if changed {
                camera.dirty = true;
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