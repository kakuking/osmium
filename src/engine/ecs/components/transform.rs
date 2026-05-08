use nalgebra::{Matrix4, UnitQuaternion, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: UnitQuaternion<f32>,
    pub scale: Vector3<f32>,
    pub model_matrix: Matrix4<f32>,

    pub dirty: bool,
}

impl Transform {
    pub fn new() -> Self {
        let position = Vector3::zeros();
        let rotation = UnitQuaternion::identity();
        let scale = Vector3::new(1.0, 1.0, 1.0);

        let translation = Matrix4::new_translation(&position);
        let rotation_mat = rotation.to_homogeneous();
        let scale_mat = Matrix4::new_nonuniform_scaling(&scale);
        
        let model_matrix = translation * rotation_mat * scale_mat;

        Self {
            position,
            rotation,
            scale,
            model_matrix,

            dirty: false,
        }
    }

    pub fn model_matrix(&self) -> Matrix4<f32> {
        let translation = Matrix4::new_translation(&self.position);
        let rotation = self.rotation.to_homogeneous();
        let scale = Matrix4::new_nonuniform_scaling(&self.scale);

        translation * rotation * scale
    }

    pub fn model_matrix_array(&self) -> [f32; 16] {
        Self::matrix_to_array(&self.model_matrix())
    }

    pub fn matrix_to_array(m: &Matrix4<f32>) -> [f32; 16] {
        let mut out = [0.0; 16];
        out.copy_from_slice(m.as_slice());
        out
    }
}