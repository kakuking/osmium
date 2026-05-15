use crate::engine::ecs::{Entity, coordinator::Coordinator};

pub struct OsmiumObject {
    pub name: String,
    pub active: bool,
    pub entity: Entity,

    pub parent: Option<Entity>,
    pub children: Vec<Entity>
}

impl OsmiumObject {
    pub fn new_isolated(name: &str, entity: Entity) -> Self {
        Self {
            name: name.to_string(),
            active: false,
            entity,

            parent: None,
            children: Vec::new()
        }
    }

    pub fn init(
        name: &str, 
        entity: Entity, 
        parent: Entity,
        coordinator: &mut Coordinator
    ) -> Self {
        let parent_object = coordinator.get_component_mut::<OsmiumObject>(parent);
        parent_object.add_child(entity);

        Self {
            name: name.to_string(),
            active: false,
            entity,

            parent: Some(parent),
            children: Vec::new()
        }
    }

    pub fn add_child(&mut self, child: Entity) {
        self.children.push(child);
    }
}