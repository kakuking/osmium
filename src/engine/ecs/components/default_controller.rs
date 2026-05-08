use nalgebra::Vector3;

pub struct DefaultController {
    pub translation: Vector3<f32>,
    pub rotation: Vector3<f32>
}

impl DefaultController {
    pub fn new() -> Self {
        Self {
            translation: Vector3::new(0.5, 0.5, 0.5),
            rotation: Vector3::zeros()
        }
    }
}