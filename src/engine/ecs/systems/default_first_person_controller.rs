use std::{
    any::Any,
    collections::HashSet,
};

use nalgebra::{UnitQuaternion, Vector3};
use winit::event::VirtualKeyCode;

use crate::engine::ecs::{
    Entity, components::{
        camera::Camera, default_first_person_controller::FirstPersonController, physics::PhysicsBody, transform::Transform
    }, system::SystemTrait, world_coordinator::WorldCoordinator
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

        let mut direction = {
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

            if dir.norm_squared() > 0.0 {
                dir.normalize()
            } else {
                dir
            }
        };

        let absolute_direction = {
            let mut dir = Vector3::<f32>::zeros();

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

        direction = {
            let adjusted_direction = rotation * direction + absolute_direction;
            if adjusted_direction.norm_squared() > 0.0 {
                adjusted_direction.normalize()
            } else {
                adjusted_direction
            }
        };

        let movement = direction * move_speed * dt;

        let rotation =
            UnitQuaternion::from_euler_angles(0.0, yaw, 0.0)
            * UnitQuaternion::from_euler_angles(pitch, 0.0, 0.0);

        let changed =
            movement != Vector3::zeros()
            || mouse_dx != 0.0
            || mouse_dy != 0.0;

        {
            
            if movement != Vector3::zeros() {
                self.update_position(
                    entity, 
                    coordinator, 
                    movement
                );
            }

            let transform: &mut Transform = coordinator.get_component_mut::<Transform>(entity);
            
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

impl FirstPersonControllerSystem {
    fn update_position(
        &self,
        entity: Entity,
        coordinator: &mut WorldCoordinator,
        movement: Vector3<f32>
    ) {
        let has_physics = coordinator.has_component::<PhysicsBody>(entity);

        let transform = coordinator.get_component_mut::<Transform>(entity);

        if has_physics {
            let body_handle = coordinator.get_component::<PhysicsBody>(entity).handle;

            if let Some(body) = coordinator.physics_world.bodies.get_mut(body_handle) {
                let current = *body.translation();

                body.set_next_kinematic_translation(
                    Vector3::new(
                        current.x + movement.x,
                        current.y + movement.y,
                        current.z + movement.z,
                    )
                );
            }
        } else {
            transform.position += movement;
        }
    }
}