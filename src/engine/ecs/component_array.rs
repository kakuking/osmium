use std::{
    any::Any, 
    collections::HashMap
};

use crate::engine::ecs::{
    Entity, 
    MAX_ENTITIES
};

pub trait ComponentArrayTrait {
    fn entity_destroyed(&mut self, entity: Entity);
    
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ComponentArray<T> {
    component_array: Vec<T>,
    entity_to_index_map: HashMap<Entity, usize>,
    index_to_entity_map: HashMap<usize, Entity>,
}

impl<T> ComponentArray<T> {
    pub fn new() -> Self {
        Self {
            component_array: Vec::with_capacity(MAX_ENTITIES),
            entity_to_index_map: HashMap::new(),
            index_to_entity_map: HashMap::new(),
        }
    }

    pub fn insert_data(&mut self, entity: Entity, component: T) {
        assert!(
            !self.entity_to_index_map.contains_key(&entity),
            "Component added to same entity more than once."
        );

        let new_index = self.component_array.len();

        self.entity_to_index_map.insert(entity, new_index);
        self.index_to_entity_map.insert(new_index, entity);
        self.component_array.push(component);
    }

    pub fn remove_data(&mut self, entity: Entity) {
        assert!(
            self.entity_to_index_map.contains_key(&entity),
            "Removing non-existent component."
        );

        let index_of_removed_entity = self.entity_to_index_map[&entity];
        let index_of_last_element = self.component_array.len() - 1;
        let entity_of_last_element = self.index_to_entity_map[&index_of_last_element];

        self.component_array.swap_remove(index_of_removed_entity);

        self.entity_to_index_map.remove(&entity);
        self.index_to_entity_map.remove(&index_of_last_element);

        if index_of_removed_entity != index_of_last_element {
            self.entity_to_index_map
                .insert(
                    entity_of_last_element, 
                    index_of_removed_entity
                );

            self.index_to_entity_map
                .insert(
                    index_of_removed_entity, 
                    entity_of_last_element
                );
        }
    }

    pub fn has_data(&self, entity: Entity) -> bool {
        self.entity_to_index_map.contains_key(&entity)
    }

    pub fn get_data(&self, entity: Entity) -> &T {
        assert!(self.entity_to_index_map.contains_key(&entity), "Retrieving non-existent component");

        &self.component_array[
            self.entity_to_index_map[&entity]
        ]
    }

    pub fn get_data_mut(&mut self, entity: Entity) -> &mut T {
        assert!(self.entity_to_index_map.contains_key(&entity), "Retrieving non-existent component");

        &mut self.component_array[
            self.entity_to_index_map[&entity]
        ]
    }
}

impl<T: 'static> ComponentArrayTrait for ComponentArray<T> {
    fn entity_destroyed(&mut self, entity: Entity) {
        if self.entity_to_index_map.contains_key(&entity) {
            self.remove_data(entity);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}