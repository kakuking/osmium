use std::{
    any::Any,
    collections::HashSet,
};

use nalgebra::UnitQuaternion;
use winit::event::MouseButton;

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
        if !coordinator.events().mouse_pressed(MouseButton::Middle) {
            return;
        }

        let (mouse_dx, mouse_dy) = coordinator.events().mouse_delta();

        {
            let controller = coordinator.get_component_mut::<OsmiumCameraController>(entity);

            controller.yaw -= mouse_dx as f32 * controller.mouse_sensitivity;
            controller.pitch -= mouse_dy as f32 * controller.mouse_sensitivity;

            controller.pitch = controller.pitch.clamp(
                -controller.pitch_limit,
                controller.pitch_limit,
            );
        }

        let (yaw, pitch) = {
            let controller = coordinator.get_component::<OsmiumCameraController>(entity);

            (
                controller.yaw,
                controller.pitch,
            )
        };

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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}