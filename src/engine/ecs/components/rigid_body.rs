use nalgebra::Vector3;

pub struct RigidBody {
    pub velocity: Vector3<f32>,
    pub acceleration: Vector3<f32>,
}

impl RigidBody {
    pub fn new() -> Self {
        Self {
            velocity: Vector3::zeros(),
            acceleration: Vector3::zeros(),
        }
    }
}