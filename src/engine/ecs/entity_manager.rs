use crate::engine::ecs::{
    Entity, 
    MAX_ENTITIES, 
    signature::Signature
};

pub struct EntityManager {
    available_entities: Vec<Entity>,
    signatures: [Signature; MAX_ENTITIES],
    live_entity_count: usize
}

impl EntityManager {
    pub fn new() -> Self {
        let mut available_entities: Vec<Entity> = Vec::new();
        let live_entity_count: usize = 0;

        for entity in 0..MAX_ENTITIES as Entity {
            available_entities.push(entity);
        }

        Self {
            available_entities,
            signatures: [ 
                Signature::new(); 
                MAX_ENTITIES
            ],
            live_entity_count
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        assert!(self.live_entity_count < MAX_ENTITIES, "Too many entities in existence");

        let id = self.available_entities
        .pop()
        .unwrap();

        self.live_entity_count += 1;

        id
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        assert!(entity < MAX_ENTITIES as u32, "Entity out of range");

        self.signatures[entity as usize].reset();

        self.available_entities.push(entity);

        self.live_entity_count -= 1;
    }

    pub fn set_signature(&mut self, entity: Entity, signature: Signature) {
        assert!(entity < MAX_ENTITIES as u32, "Entity out of range");

        self.signatures[entity as usize] = signature;
    }

    pub fn get_signature(&self, entity: Entity) -> Signature {
        assert!(entity < MAX_ENTITIES as u32, "Entity out of range");

        self.signatures[entity as usize]
    }
}