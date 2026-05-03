use nalgebra::Vector3;

pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vector3::zeros(),
            rotation: Vector3::zeros(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}