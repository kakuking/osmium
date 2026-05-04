use rapier3d::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct PhysicsBody {
    pub handle: RigidBodyHandle,
}

#[derive(Clone, Copy, Debug)]
pub struct PhysicsCollider {
    pub handle: ColliderHandle,
}

#[derive(Clone, Copy, Debug)]
pub enum PhysicsBodyType {
    Dynamic,
    Fixed,
}

#[derive(Clone, Copy, Debug)]
pub struct PhysicsBodyConfig {
    pub body_type: PhysicsBodyType,
    pub half_extents: [f32; 3],
}

impl PhysicsBodyConfig {
    pub fn dynamic_box(half_extents: [f32; 3]) -> Self {
        Self {
            body_type: PhysicsBodyType::Dynamic,
            half_extents,
        }
    }

    pub fn fixed_box(half_extents: [f32; 3]) -> Self {
        Self {
            body_type: PhysicsBodyType::Fixed,
            half_extents,
        }
    }
}