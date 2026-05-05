
use crate::engine::{
    ecs::{
        ComponentType, 
        Entity, 
        components::{
            camera::Camera, 
            renderable::MeshRenderable, 
            transform::Transform
        }, 
        signature::Signature, 
        system::SystemTrait, 
        system_manager::SystemManager, 
        systems::{camera::CameraSystem, render::RenderSystem}, world_coordinator::WorldCoordinator
    }, renderer::global_resources::{
        RenderGlobals
    }, 
    scene::render_item::RenderItem, 
    window::event_manager::{
        EngineEvent, 
        EventManager
    }
};

pub struct Coordinator {
    world_coordinator: WorldCoordinator,
    system_manager: SystemManager
}

impl Coordinator {
    pub fn new() -> Self {
        Self {
            world_coordinator: WorldCoordinator::new(),
            system_manager: SystemManager::new()
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.world_coordinator.entity_manager.create_entity()
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.world_coordinator.entity_manager.destroy_entity(entity);
        self.world_coordinator.component_manager.entity_destroyed(entity);
        self.system_manager.entity_destroyed(entity);
    }

    pub fn register_component<T: 'static>(&mut self) {
        self.world_coordinator.component_manager.register_component::<T>();
    }

    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        self.world_coordinator.component_manager.add_component(entity, component);

        let mut signature = self.world_coordinator.entity_manager.get_signature(entity);

        signature.set(self.world_coordinator.component_manager.get_component_type::<T>() as usize, true);

        self.world_coordinator.entity_manager.set_signature(entity, signature);
        self.system_manager.entity_signature_changed(entity, signature);
    }

    pub fn remove_component<T: 'static>(&mut self, entity: Entity) {
        self.world_coordinator.component_manager.remove_component::<T>(entity);

        
        let mut signature = self.world_coordinator.entity_manager.get_signature(entity);
        signature.set(self.world_coordinator.component_manager.get_component_type::<T>() as usize, false);

        self.world_coordinator.entity_manager.set_signature(entity, signature);
        self.system_manager.entity_signature_changed(entity, signature);
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> &T {
        self.world_coordinator.component_manager.get_component::<T>(entity)
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> &mut T {
        self.world_coordinator.component_manager.get_component_mut::<T>(entity)
    }

    pub fn get_component_type<T: 'static>(&self) -> ComponentType {
        self.world_coordinator.component_manager.get_component_type::<T>()
    }

    pub fn register_system<T: 'static + SystemTrait + Default>(&mut self) -> &mut T {
        self.system_manager.register_system::<T>()
    }

    pub fn get_system<T: 'static + SystemTrait + Default>(&self) -> &T {
        self.system_manager.get_system::<T>()
    }

    pub fn get_system_mut<T: 'static + SystemTrait + Default>(&mut self) -> &mut T {
        self.system_manager.get_system_mut::<T>()
    }

    pub fn set_system_signature<T: 'static + SystemTrait + Default>(&mut self, signature: Signature) {
        self.system_manager.set_signature::<T>(signature);
    }

    pub fn initialize_systems(&mut self) {
        self.system_manager.initialize_all_systems(
            &mut self.world_coordinator
        );
    }

    pub fn update_systems(&mut self, dt: f32) {
        self.system_manager.update_all_systems(
            &mut self.world_coordinator, 
            dt
        );
    }

    pub fn events(&self) -> &EventManager {
        self.world_coordinator.events()
    }

    pub fn events_mut(&mut self) -> &mut EventManager {
        self.world_coordinator.events_mut()
    }

    pub fn send_event(&mut self, event: EngineEvent) {
        self.world_coordinator.send_event(event);
    }

    pub fn clear_frame_events(&mut self) {
        self.world_coordinator.clear_frame_events();
    }

    pub fn get_render_items(&self) -> Vec<RenderItem> {
        let render_system = self.get_system::<RenderSystem>();

        render_system
            .entities
            .iter()
            .map(|entity| {
                let mesh_renderable = self.get_component::<MeshRenderable>(*entity);
                let transform = self.get_component::<Transform>(*entity);
                
                RenderItem {
                    mesh: mesh_renderable.mesh,
                    material: mesh_renderable.material,

                    model_matrix: transform.model_matrix()
                }
            })
            .collect()
    }

    pub fn get_global_resources(&mut self, aspect_ratio: f32) -> RenderGlobals {
        let camera_entities: Vec<_> = self
            .get_system::<CameraSystem>()
            .entities
            .iter()
            .copied()
            .collect();

        for entity in camera_entities {
            let active = self.get_component_mut::<Camera>(entity).active;
            if active {
                let transform = *self.get_component::<Transform>(entity);

                let camera = self.get_component_mut::<Camera>(entity);

                let camera_data = camera.get_camera_data(
                    aspect_ratio, 
                    &transform
                );

                return RenderGlobals {
                    camera: camera_data,
                    point_lights: Vec::new(),
                    directional_lights: Vec::new(),
                };
            }
        }

        panic!("No active camera");
    }
}