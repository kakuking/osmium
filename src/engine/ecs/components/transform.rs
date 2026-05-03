use nalgebra::Vector3;

#[derive(Debug, Clone, Copy)]
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

    pub fn model_matrix(&self) -> [[f32; 4]; 4] {
        // temporary example
        [
            [1.0, 0.0, 0.0, self.position[0]],
            [0.0, 1.0, 0.0, self.position[1]],
            [0.0, 0.0, 1.0, self.position[2]],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }
}