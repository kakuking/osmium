use std::{any::Any, collections::HashSet};

use rapier3d::prelude::*;

use crate::engine::ecs::{
    Entity,
    components::{
        physics::{
            PhysicsBody,
            PhysicsBodyConfig,
            PhysicsBodyType,
            PhysicsCollider,
        },
        transform::Transform,
    },
    system::SystemTrait,
    world_coordinator::WorldCoordinator,
};

#[derive(Default)]
pub struct PhysicsSystem {
    pub entities: HashSet<Entity>,
}

impl PhysicsSystem {
    fn initialize_entity(&self, entity: Entity, world: &mut WorldCoordinator) {
        if world.has_component::<PhysicsBody>(entity) {
            return;
        }

        let transform = *world.get_component::<Transform>(entity);
        let config = *world.get_component::<PhysicsBodyConfig>(entity);

        let body_builder = match config.body_type {
            PhysicsBodyType::Dynamic => RigidBodyBuilder::dynamic(),
            PhysicsBodyType::Fixed => RigidBodyBuilder::fixed(),
        };

        let rigid_body = body_builder
            .translation(vector![
                transform.position.x,
                transform.position.y,
                transform.position.z,
            ])
            .build();

        let physics = &mut world.physics_world;

        let body_handle = physics.bodies.insert(rigid_body);

        let collider = ColliderBuilder::cuboid(
            config.half_extents[0],
            config.half_extents[1],
            config.half_extents[2],
        )
        .build();

        let collider_handle = physics.colliders.insert_with_parent(
            collider,
            body_handle,
            &mut physics.bodies,
        );

        world.add_component(entity, PhysicsBody {
            handle: body_handle,
        });

        world.add_component(entity, PhysicsCollider {
            handle: collider_handle,
        });
    }
}

impl SystemTrait for PhysicsSystem {
    fn entities(&self) -> &HashSet<Entity> {
        &self.entities
    }

    fn entities_mut(&mut self) -> &mut HashSet<Entity> {
        &mut self.entities
    }

    fn initialize(&self, world: &mut WorldCoordinator) {
        let entities: Vec<Entity> = self.entities.iter().copied().collect();

        for entity in entities {
            self.initialize_entity(entity, world);
        }
    }

    fn update_all_entities(&self, world: &mut WorldCoordinator, dt: f32) {
        for entity in self.entities.iter().copied() {
            self.initialize_entity(entity, world);
        }

        world.physics_world.step(dt);

        for entity in self.entities.iter().copied() {
            self.update(entity, world, dt);
        }
    }

    fn update(&self, entity: Entity, world: &mut WorldCoordinator, _dt: f32) {
        if !world.has_component::<PhysicsBody>(entity) {
            return;
        }

        let body_handle = world.get_component::<PhysicsBody>(entity).handle;

        let Some(pos) = world
            .physics_world
            .bodies
            .get(body_handle)
            .map(|body| *body.translation()) 
        else {
            return;
        };

        let transform = world.get_component_mut::<Transform>(entity);

        transform.position.x = pos.x;
        transform.position.y = pos.y;
        transform.position.z = pos.z;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}