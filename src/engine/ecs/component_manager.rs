use std::{any::type_name, collections::HashMap};

use crate::engine::ecs::{
    ComponentType, 
    Entity, 
    component_array::{
        ComponentArray, 
        ComponentArrayTrait
    }
};

pub struct ComponentManager {
    component_types: HashMap<String, ComponentType>,
    component_arrays: HashMap<String, Box<dyn ComponentArrayTrait>>,
    next_component_type: ComponentType,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            component_types: HashMap::new(),
            component_arrays: HashMap::new(),
            next_component_type: 0u8
        }
    }

    pub fn register_component<T: 'static>(&mut self) {
        let type_name: String = type_name::<T>().to_string();

        assert!(
            !self.component_types.contains_key(&type_name),
            "Registering component type more than once"
        );


        self.component_types.insert(
            type_name.clone(), 
            self.next_component_type
        );

        self.component_arrays.insert(
            type_name, 
            Box::new(
                ComponentArray::<T>::new()
            )
        );

        self.next_component_type += 1;
    }

    pub fn get_component_type<T: 'static>(&self) -> ComponentType {
        let type_name: String = type_name::<T>().to_string();

        assert!(
            self.component_types.contains_key(&type_name),
            "Component not registered before use"
        );

        self.component_types[&type_name]
    }

    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        self.get_component_array_mut::<T>()
            .insert_data(entity, component);
    }

    pub fn remove_component<T: 'static>(&mut self, entity: Entity) {
        self.get_component_array_mut::<T>()
            .remove_data(entity);
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> &T {
        self.get_component_array::<T>()
            .get_data(entity)
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> &mut T {
        self.get_component_array_mut::<T>()
            .get_data_mut(entity)
    }

    pub fn entity_destroyed(&mut self, entity: Entity) {
        for component_array in self.component_arrays.values_mut() {
            component_array.entity_destroyed(entity);
        }
    }

    fn get_component_array<T: 'static>(&self) -> &ComponentArray<T> {
        let type_name: String = type_name::<T>().to_string();
        
        assert!(
            self.component_types.contains_key(&type_name),
            "Component not registered before use",
        );

        self.component_arrays[&type_name]
            .as_any()
            .downcast_ref::<ComponentArray<T>>()
            .expect("Component array type mismatch")
    }

    fn get_component_array_mut<T: 'static>(&mut self) -> &mut ComponentArray<T> {
        let type_name: String = type_name::<T>().to_string();
        
        assert!(
            self.component_types.contains_key(&type_name),
            "Component not registered before use",
        );

        self.component_arrays
            .get_mut(&type_name)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<ComponentArray<T>>()
            .expect("Component array type mismatch")
    }
}