use crate::engine::{ecs::components::transform::Transform, renderer::global_resources::CameraGpuData};

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

        let model = glam::Mat4::IDENTITY;
        let view = model.inverse();

        let mut proj = glam::Mat4::perspective_rh_gl(
            fov_y_radians,
            1.0,
            near,
            far,
        );

        proj.y_axis.y *= -1.0;

        let view_proj = proj * view;

        let data =  CameraGpuData {
            view: view.to_cols_array_2d(),
            proj: proj.to_cols_array_2d(),
            view_proj: view_proj.to_cols_array_2d(),
            camera_pos: [
                1.0,1.0,1.0,1.0,
            ]
        };
        
        Self {
            active,
            fov_y_radians,
            near,
            far,

            dirty: true,
            data
        }
    }

    fn rebuild_camera_data(
        &mut self,
        aspect_ratio: f32,
        transform: &Transform
    ) -> CameraGpuData {
        let model = glam::Mat4::from_cols_array_2d(&transform.model_matrix());
        let view = model.inverse();

        let mut proj = glam::Mat4::perspective_rh_gl(
            self.fov_y_radians,
            aspect_ratio,
            self.near,
            self.far,
        );

        proj.y_axis.y *= -1.0;

        let view_proj = proj * view;

        self.data =  CameraGpuData {
            view: view.to_cols_array_2d(),
            proj: proj.to_cols_array_2d(),
            view_proj: view_proj.to_cols_array_2d(),
            camera_pos: [
                transform.position.x,
                transform.position.y,
                transform.position.z,
                1.0,
            ]
        };

        self.data.clone()
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