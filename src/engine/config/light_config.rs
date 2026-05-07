use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct DirectionalLightConfig {
    pub direction: [f32; 3],
    pub color: [f32; 3]
}

impl Default for DirectionalLightConfig {
    fn default() -> Self {
        Self {
            direction: [1.0, 1.0, 1.0],
            color: [1.0, 1.0, 1.0]
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct PointLightConfig {
    pub position: [f32; 3],
    pub color: [f32; 3]
}

impl Default for PointLightConfig {
    fn default() -> Self {
        Self {
            position: [1.0, 1.0, 1.0],
            color: [1.0, 1.0, 1.0]
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LightConfig {
    Point(PointLightConfig),
    Directional(DirectionalLightConfig),
}

impl LightConfig {
    pub fn new(directional: bool) -> Self {
        if directional {
            Self::Directional(DirectionalLightConfig::default())   
        } else {
            Self::Point(PointLightConfig::default())
        }
    }

    pub fn get_color(&self) -> [f32; 3] {
        match self {
            Self::Directional(l) => l.color.clone(),
            Self::Point(l) => l.color.clone(),
        }
    }
}