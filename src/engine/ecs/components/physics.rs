use rapier3d::prelude::*;

use crate::engine::scene::mesh::{Mesh, OsmiumVertex};

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
    KinematicPositionBased
}

#[derive(Clone, Copy, Debug)]
pub struct PhysicsBodyConfig {
    pub body_type: PhysicsBodyType,
    pub half_extents: [f32; 3],
}

impl PhysicsBodyConfig {
    pub fn from_extents(
        half_extents: [f32; 3], 
        body_type: PhysicsBodyType
    ) -> Self {
        Self {
            body_type,
            half_extents,
        }
    }

    pub fn from_vertices(
        vertices: &[OsmiumVertex], 
        body_type: PhysicsBodyType
    ) -> Self {
        let mut min = [f32::MAX; 3];
        let mut max = [f32::MIN; 3];

        for v in vertices {
            for i in 0..3 {
                min[i] = min[i].min(v.position[i]);
                max[i] = max[i].max(v.position[i]);
            }
        }

        let half_extents = [
            (max[0] - min[0]) * 0.5,
            (max[1] - min[1]) * 0.5,
            (max[2] - min[2]) * 0.5,
        ];

        Self {
            body_type,
            half_extents
        }
    }

    pub fn from_mesh(
        mesh: &Mesh,
        body_type: PhysicsBodyType
    ) -> Self {
        Self::from_vertices(
            &mesh.get_vertices(), 
            body_type
        )
    }
}