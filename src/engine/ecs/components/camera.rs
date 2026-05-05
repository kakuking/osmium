use nalgebra::{Matrix4, Perspective3};

use crate::engine::{
    ecs::components::transform::Transform, 
    renderer::global_resources::CameraGpuData
};

pub struct Camera {
    pub active: bool,
    pub fov_y_radians: f32,
    pub near: f32,
    pub far: f32,

    pub dirty: bool,
    pub data: CameraGpuData
}

impl Camera {
    pub fn new(active: bool) -> Self {
        let fov_y_radians = 60.0_f32.to_radians();
        let near = 0.01;
        let far = 100.0;

        let view = Matrix4::identity();

        let mut proj = Perspective3::new(
            1.0,
            fov_y_radians,
            near,
            far,
        )
        .to_homogeneous();

        proj[(1, 1)] *= -1.0;

        let view_proj = proj * view;

        let data = CameraGpuData {
            view: Transform::matrix_to_array(&view),
            proj: Transform::matrix_to_array(&proj),
            view_proj: Transform::matrix_to_array(&view_proj),
            camera_pos: [0.0, 0.0, 0.0, 1.0],
        };

        Self {
            active,
            fov_y_radians,
            near,
            far,
            dirty: true,
            data,
        }
    }

    fn rebuild_camera_data(
        &mut self,
        aspect_ratio: f32,
        transform: &Transform
    ) -> CameraGpuData {
        let model = transform.model_matrix();

        let view = model
            .try_inverse()
            .unwrap_or_else(Matrix4::identity);

        let mut proj = Perspective3::new(
            aspect_ratio,
            self.fov_y_radians,
            self.near,
            self.far,
        )
        .to_homogeneous();

        proj[(1, 1)] *= -1.0;

        let view_proj = proj * view;

        self.data = CameraGpuData {
            view: Transform::matrix_to_array(&view),
            proj: Transform::matrix_to_array(&proj),
            view_proj: Transform::matrix_to_array(&view_proj),
            camera_pos: [
                transform.position.x,
                transform.position.y,
                transform.position.z,
                1.0,
            ],
        };

        self.data
    }

    pub fn get_camera_data(
        &mut self,
        aspect_ratio: f32,
        transform: &Transform
    ) -> CameraGpuData {
        if self.dirty {
            self.dirty = false;
            self.rebuild_camera_data(aspect_ratio, transform)
        } else {
            self.data.clone()
        }
    }
}