use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct MeshConfig {
    pub filepath: String,
    pub material: String,
    pub heightmap: Option<String>,
}

impl MeshConfig {
    pub fn new() -> Self {
        Self {
            filepath: "./resources/Cube.obj".into(),
            material: "default".into(),
            heightmap: None
        }
    }
}
