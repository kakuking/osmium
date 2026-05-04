use std::{any::type_name, collections::HashMap};

use crate::engine::ecs::{
    Entity, 
    signature::Signature, 
    system::SystemTrait, world_coordinator::WorldCoordinator
};

pub struct SystemManager {
    signatures: HashMap<String, Signature>,
    systems: HashMap<String, Box<dyn SystemTrait>>,
    pub execution_order: Vec<String>,
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            signatures: HashMap::new(),
            systems: HashMap::new(),
            execution_order: Vec::new()
        }
    }

    pub fn register_system<T>(&mut self)
        -> &mut T
    where
        T: SystemTrait + Default + 'static
    {
        let type_name: String = type_name::<T>().to_string();

        assert!(!self.systems.contains_key(&type_name), "Registering system more than once");

        let system = Box::new(T::default());
        self.systems.insert(type_name.clone(), system);

        self.execution_order.push(type_name.clone());

        self.systems
            .get_mut(&type_name)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<T>()
            .expect("System type mismatch")
    }

    pub fn set_signature<T: 'static>(&mut self, signature: Signature) {
        let type_name: String = type_name::<T>().to_string();

        assert!(
            self.systems.contains_key(&type_name),
            "System used before registered: {}",
            type_name
        );

        self.signatures.insert(type_name, signature);
    }

    pub fn entity_destroyed(&mut self, entity: Entity) {
        for system in self.systems.values_mut() {
            system.entities_mut().remove(&entity);
        }
    }

    pub fn entity_signature_changed(
        &mut self,
        entity: Entity,
        entity_signature: Signature,
    ) {
        for (type_id, system) in self.systems.iter_mut() {
            let system_signature = self.signatures[type_id];

            if (entity_signature & system_signature) == system_signature {
                system.entities_mut().insert(entity);
            } else {
                system.entities_mut().remove(&entity);
            }
        }
    }

    pub fn get_system<T: 'static>(&self) -> &T {
        let type_name: String = type_name::<T>().to_string();

        self.systems[&type_name]
            .as_any()
            .downcast_ref::<T>()
            .expect("System type mismatch")
    }

    pub fn get_system_mut<T: 'static>(&mut self) -> &mut T {
        let type_name: String = type_name::<T>().to_string();

        self.systems
            .get_mut(&type_name)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<T>()
            .expect("System type mismatch")
    }

    pub fn initialize_all_systems(
        &mut self,
        coordinator: &mut WorldCoordinator
    ) {
        for (_, system) in &mut self.systems {
            system.initialize(coordinator);
        }
    }

    pub fn update_all_systems(
        &mut self, 
        coordinator: &mut WorldCoordinator, 
        dt: f32
    ) {
        for (_, system) in &mut self.systems {
            system.update_all_entities(coordinator, dt);
        }
    }
}