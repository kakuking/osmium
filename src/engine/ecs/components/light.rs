use nalgebra::{Matrix4, Orthographic3, Perspective3};

use crate::engine::{
    config::light_config::LightConfig, 
    ecs::components::transform::Transform, 
    renderer::global_resources::LightGpuData
};

pub struct Light {
    config: LightConfig,

    pub dirty: bool,
    pub data: LightGpuData,
}

impl Light {
    pub fn new(config: LightConfig) -> Self {
        let data = Self::rebuild_light_data(
            &config, 
            &mut Transform::new()
        );

        Self {
            config,

            dirty: true,
            data,
        }
    }

    pub fn get_view_proj(&self) -> [f32; 16] {
        self.data.view_proj.clone()
    }

    fn rebuild_light_data(
        config: &LightConfig,
        transform: &Transform
    ) -> LightGpuData {
        let model = transform.model_matrix();

        let view = model
            .try_inverse()
            .unwrap_or_else(Matrix4::identity);

        let mut proj = match config {
            LightConfig::Point(_cfg) => {
                Perspective3::new(
                    1.0,
                    60.0f32.to_radians(),
                    0.01,
                    20.0
                )
                .to_homogeneous()
            },
            LightConfig::Directional(_cfg) => {
                let center_x = 0.0;
                let center_y = 0.0;

                let half_height = (10.0 + 10.0) * 0.5;
                let half_width = half_height * 1.0;

                Orthographic3::new(
                    center_x - half_width,
                    center_x + half_width,
                    center_y - half_height,
                    center_y + half_height,
                    0.01,
                    20.0,
                )
                .to_homogeneous()
            }
        };

        let vulkan_clip = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, -1.0, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.5,
            0.0, 0.0, 0.0, 1.0,
        );

        proj = vulkan_clip * proj;

        let view_proj = proj * view;

        LightGpuData {
            view_proj: Transform::matrix_to_array(&view_proj),
            color: config.get_color(),
            _pad: 0.0
        }
    }

    pub fn get_light_data(
        &mut self,
        transform: &Transform
    ) -> LightGpuData {
        if self.dirty {
            self.dirty = false;
            self.data = Self::rebuild_light_data(
                &self.config,
                transform
            );
        }

        self.data.clone()
    }
}