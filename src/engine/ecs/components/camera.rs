use nalgebra::{Matrix4, Orthographic3, Perspective3};

use crate::engine::{
    config::camera_config::CameraConfig, 
    ecs::components::transform::Transform, 
    renderer::global_resources::CameraGpuData
};

pub struct Camera {
    pub active: bool,
    config: CameraConfig,

    pub dirty: bool,
    pub data: CameraGpuData,
}

impl Camera {
    pub fn new(config: CameraConfig, active: bool) -> Self {
        let data = Self::rebuild_camera_data(
            &config, 
            1.0,
            &Transform::new()
        );

        Self {
            active,
            config,

            dirty: true,
            data,
        }
    }

    fn rebuild_camera_data(
        config: &CameraConfig,
        aspect_ratio: f32,
        transform: &Transform
    ) -> CameraGpuData {
        let model = transform.model_matrix();

        let view = model
            .try_inverse()
            .unwrap_or_else(Matrix4::identity);

        let mut proj = match config {
            CameraConfig::Perspective(cfg) => {
                Perspective3::new(
                    aspect_ratio,
                    cfg.fov_y_radians,
                    cfg.near,
                    cfg.far
                )
                .to_homogeneous()
            },
            CameraConfig::Orthographic(cfg) => {
                let center_x = (cfg.left + cfg.right) * 0.5;
                let center_y = (cfg.bottom + cfg.top) * 0.5;

                let half_height = (cfg.top - cfg.bottom) * 0.5;
                let half_width = half_height * aspect_ratio;

                Orthographic3::new(
                    center_x - half_width,
                    center_x + half_width,
                    center_y - half_height,
                    center_y + half_height,
                    -cfg.far,
                    -cfg.near,
                )
                .to_homogeneous()
            }
        };

        let vulkan_clip = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, -1.0, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        proj = vulkan_clip * proj;

        let view_proj = proj * view;

        CameraGpuData {
            view: Transform::matrix_to_array(&view),
            proj: Transform::matrix_to_array(&proj),
            view_proj: Transform::matrix_to_array(&view_proj),
            camera_pos: [
                transform.position.x,
                transform.position.y,
                transform.position.z,
                1.0,
            ],
        }
    }

    pub fn get_camera_data(
        &mut self,
        aspect_ratio: f32,
        transform: &Transform
    ) -> CameraGpuData {
        if self.dirty {
            self.dirty = false;
            self.data = Self::rebuild_camera_data(
                &self.config,
                aspect_ratio,
                transform
            );
        }
        self.data.clone()
    }
}