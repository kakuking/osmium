use nalgebra::Vector3;

pub struct MovementSpeeds {
    pub translation: Vector3<f32>,
    pub rotation: Vector3<f32>
}

impl MovementSpeeds {
    pub fn new() -> Self {
        Self {
            translation: Vector3::new(0.5, 0.5, 0.5),
            rotation: Vector3::zeros()
        }
    }
}