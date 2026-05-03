use std::{
    any::Any, 
    collections::HashSet
};

use crate::engine::ecs::{
    Entity, 
    components::{
        gravity::Gravity, 
        rigid_body::RigidBody, 
        transform::Transform
    }, 
    coordinator::WorldCoordinator, 
    system::SystemTrait
};


#[derive(Default)]
pub struct PhysicsSystem {
    pub entities: HashSet<Entity>,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            entities: HashSet::new(),
        }
    }
}

impl SystemTrait for PhysicsSystem {
    fn entities(&self) -> &HashSet<Entity> {
        &self.entities
    }

    fn entities_mut(&mut self) -> &mut HashSet<Entity> {
        &mut self.entities
    }

    fn update(&self, coordinator: &mut WorldCoordinator, dt: f32) {
        for entity in self.entities.iter().copied() {
            let gravity = coordinator.get_component::<Gravity>(entity).g;

            {
                let rigid_body = coordinator.get_component_mut::<RigidBody>(entity);
                rigid_body.velocity.y += gravity * dt;
            }

            let velocity = coordinator.get_component::<RigidBody>(entity).velocity;

            {
                let t = coordinator.get_component_mut::<Transform>(entity);
                t.position += velocity * dt;
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