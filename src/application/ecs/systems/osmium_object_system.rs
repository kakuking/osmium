use std::{
    any::Any,
    collections::HashSet,
};

use crate::{
    application::ecs::components::osmium_object::OsmiumObject, engine::ecs::{
        Entity, 
        system::SystemTrait, 
        world_coordinator::WorldCoordinator
    }
};

#[derive(Default)]
pub struct OsmiumObjectSystem {
    pub entities: HashSet<Entity>,

    pub roots: Vec<Entity>,
}

impl OsmiumObjectSystem {
    pub fn new() -> Self {
        Self {
            entities: HashSet::new(),
            roots: Vec::new()
        }
    }
}

impl SystemTrait for OsmiumObjectSystem {
    fn entities(&self) -> &HashSet<Entity> {
        &self.entities
    }

    fn entities_mut(&mut self) -> &mut HashSet<Entity> {
        &mut self.entities
    }

    fn update(&self, _entity: Entity, _coordinator: &mut WorldCoordinator, _dt: f32) { }

    fn initialize(&mut self, world: &mut WorldCoordinator) {
        let entities: Vec<Entity> = self.entities().iter().copied().collect();

        for entity in entities {
            let object = world.get_component::<OsmiumObject>(entity);

            if object.parent.is_none() {
                self.roots.push(entity);
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

impl OsmiumObjectSystem {
    pub fn asdasd(&self) {

    }
}