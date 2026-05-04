use crate::engine::{
    ecs::{
        ComponentType, 
        Entity, 
        component_manager::ComponentManager, 
        entity_manager::EntityManager
    }, 
    physics::physics_world::PhysicsWorld, 
    window::event_manager::{
        EngineEvent, 
        EventManager
    }
};


pub struct WorldCoordinator {
    pub component_manager: ComponentManager,
    pub entity_manager: EntityManager,
    pub events: EventManager,
    pub physics_world: PhysicsWorld
}

impl WorldCoordinator {
    pub fn new() -> Self {
        Self {
            component_manager: ComponentManager::new(),
            entity_manager: EntityManager::new(),
            events: EventManager::default(),
            physics_world: PhysicsWorld::new()
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

    pub fn events(&self) -> &EventManager {
        &self.events
    }

    pub fn events_mut(&mut self) -> &mut EventManager {
        &mut self.events
    }

    pub fn send_event(&mut self, event: EngineEvent) {
        self.events.send(event);
    }

    pub fn clear_frame_events(&mut self) {
        self.events.clear_frame_events();
    }
    
    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        self.component_manager.add_component(entity, component);

        let mut signature = self.entity_manager.get_signature(entity);
        signature.set(self.component_manager.get_component_type::<T>() as usize, true);
        self.entity_manager.set_signature(entity, signature);
    }

    pub fn has_component<T: 'static>(&self, entity: Entity) -> bool {
        self.component_manager.has_component::<T>(entity)
    }
}
