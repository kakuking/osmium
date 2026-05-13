use std::{
    any::Any,
    collections::HashSet,
};

use nalgebra::{UnitQuaternion, Vector3};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    application::ecs::components::osmium_camera::OsmiumCameraController, 
        engine::ecs::{
        Entity, components::{
            camera::Camera, 
            transform::Transform
        }, 
        system::SystemTrait, 
        world_coordinator::WorldCoordinator
    }
};

#[derive(Default)]
pub struct OsmiumCameraControllerSystem {
    pub entities: HashSet<Entity>,
}

impl OsmiumCameraControllerSystem {
    pub fn new() -> Self {
        Self {
            entities: HashSet::new(),
        }
    }
}

impl SystemTrait for OsmiumCameraControllerSystem {
    fn entities(&self) -> &HashSet<Entity> {
        &self.entities
    }

    fn entities_mut(&mut self) -> &mut HashSet<Entity> {
        &mut self.entities
    }

    // for now only rotation
    fn update(&self, entity: Entity, coordinator: &mut WorldCoordinator, _dt: f32) {

        self.scroll_movement(entity, coordinator);

        let (mouse_dx, mouse_dy) = coordinator.events().mouse_delta();

        let shift_pressed =
            coordinator.events().key_pressed(VirtualKeyCode::LShift) ||
            coordinator.events().key_pressed(VirtualKeyCode::RShift);

        if shift_pressed {
            self.pan_movement(entity, coordinator, mouse_dx, mouse_dy);
        } else {
            self.rotate_movement(entity, coordinator, mouse_dx, mouse_dy);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl OsmiumCameraControllerSystem {
    fn scroll_movement(&self, entity: Entity, coordinator: &mut WorldCoordinator) {
        let scroll = coordinator.events().scroll_delta();

        if scroll != 0.0 {
            let zoom_sensitivity = {
                let controller = coordinator.get_component::<OsmiumCameraController>(entity);
                controller.zoom_sensitivity
            };

            {
                let transform: &mut Transform =
                    coordinator.get_component_mut::<Transform>(entity);

                let forward = transform.rotation * -nalgebra::Vector3::z();

                transform.position +=
                    forward * scroll as f32 * zoom_sensitivity;

                transform.dirty = true;
            }

            {
                let camera = coordinator.get_component_mut::<Camera>(entity);
                camera.dirty = true;
            }
        }
    }

    fn pan_movement(
        &self, 
        entity: Entity, 
        coordinator: &mut WorldCoordinator,
        mouse_dx: f64, mouse_dy: f64
    ) {
        if !coordinator.events().mouse_pressed(MouseButton::Left) {
            return;
        }

        if mouse_dx == 0.0 && mouse_dy == 0.0 {
            return;
        }
        
        let pan_sensitivity = {
            let controller = coordinator.get_component::<OsmiumCameraController>(entity);
            controller.pan_sensitivity
        };

        {
            let transform: &mut Transform = coordinator.get_component_mut::<Transform>(entity);

            let right = transform.rotation * Vector3::x();
            let up = transform.rotation * Vector3::y();

            transform.position +=
                (-right * mouse_dx as f32 + up * mouse_dy as f32) * pan_sensitivity;

            transform.dirty = true;
        }

        {
            let camera = coordinator.get_component_mut::<Camera>(entity);
            camera.dirty = true;
        }

        return;
    }

    fn rotate_movement(
        &self, 
        entity: Entity, 
        coordinator: &mut WorldCoordinator,
        mouse_dx: f64, mouse_dy: f64
    ) {
        if !coordinator.events().mouse_pressed(MouseButton::Middle) {
            return;
        }
        
        if mouse_dx == 0.0 && mouse_dy == 0.0 {
            return;
        }

        let controller = coordinator.get_component_mut::<OsmiumCameraController>(entity);

        controller.yaw -= mouse_dx as f32 * controller.mouse_sensitivity;
        controller.pitch -= mouse_dy as f32 * controller.mouse_sensitivity;

        controller.pitch = controller.pitch.clamp(
            -controller.pitch_limit,
            controller.pitch_limit,
        );

        let (yaw, pitch) = (controller.yaw, controller.pitch);

        let rotation =
            UnitQuaternion::from_euler_angles(0.0, yaw, 0.0) *
            UnitQuaternion::from_euler_angles(pitch, 0.0, 0.0);

        let changed = mouse_dx != 0.0 || mouse_dy != 0.0;

        {
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
}