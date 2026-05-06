use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct OrthographicCameraConfig {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for OrthographicCameraConfig {
    fn default() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.01,
            far: 100.0,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct PerspectiveCameraConfig {
    pub aspect: f32,
    pub fov_y_radians: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for PerspectiveCameraConfig {
    fn default() -> Self {
        Self {
            aspect: 1.0,
            fov_y_radians: 60.0_f32.to_radians(),
            near: 0.01,
            far: 100.0,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CameraConfig {
    Perspective(PerspectiveCameraConfig),
    Orthographic(OrthographicCameraConfig),
}

impl CameraConfig {
    pub fn new(perspective: bool) -> Self {
        if perspective {
            Self::Perspective(PerspectiveCameraConfig::default())   
        } else {
            Self::Orthographic(OrthographicCameraConfig::default())
        }
    }
}