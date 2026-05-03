use nalgebra::Vector3;

pub struct RigidBody {
    pub velocity: Vector3<f32>,
    pub acceleration: Vector3<f32>,
    // pub velocity: [f32; 3],
    // pub acceleration: [f32; 3],
}

impl RigidBody {
    pub fn new() -> Self {
        Self {
            // velocity: [0.0, 0.0, 0.0],
            // acceleration: [0.0, 0.0, 0.0],
            velocity: Vector3::zeros(),
            acceleration: Vector3::zeros(),
        }
    }
}