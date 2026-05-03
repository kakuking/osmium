use crate::engine::ecs::{
    ComponentType, Entity, component_manager::ComponentManager, entity_manager::EntityManager, signature::Signature, system::SystemTrait, system_manager::SystemManager
};

pub struct WorldCoordinator {
    component_manager: ComponentManager,
    entity_manager: EntityManager,
}

impl WorldCoordinator {
    pub fn new() -> Self {
        Self {
            component_manager: ComponentManager::new(),
            entity_manager: EntityManager::new()
        }
    }
}

impl WorldCoordinator {
    pub fn get_component<T: 'static>(&self, entity: Entity) -> &T {
        self.component_manager.get_component::<T>(entity)
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> &mut T {
        self.component_manager.get_component_mut::<T>(entity)
    }

    pub fn get_component_type<T: 'static>(&self) -> ComponentType {
        self.component_manager.get_component_type::<T>()
    }
}


pub struct Coordinator {
    world_coordinator: WorldCoordinator,
    system_manager: SystemManager
}

impl Coordinator {
    pub fn new() -> Self {
        Self {
            // component_manager: ComponentManager::new(),
            // entity_manager: EntityManager::new(),
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

    pub fn update_systems(&mut self, dt: f32) {
        self.system_manager.update_all_systems(
            &mut self.world_coordinator, 
            dt
        );
    }
}