use serde::Deserialize;


fn default_base_color() -> [f32; 4] {
    [1.0, 1.0, 1.0, 1.0]
}

fn default_roughness() -> f32 {
    0.5
}

#[derive(Debug, Deserialize)]
pub struct MaterialParams {
    #[serde(default="default_base_color")]
    pub base_color: [f32; 4],
    #[serde(default="default_roughness")]    
    pub roughness: f32,
    #[serde(default)]    
    pub metallic: f32,
}

#[derive(Debug, Deserialize)]
pub struct MaterialTextures {
    pub albedo: Option<String>,
    pub normal: Option<String>,
    pub roughness: Option<String>,
    pub metallic: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MaterialConfig {
    pub name: String,
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub params: MaterialParams,
    pub textures: MaterialTextures
}

impl MaterialConfig {
    pub fn new() -> Self {
        Self {
            name: "default_texture".into(),
            vertex_shader: "./shaders/vertex.glsl".into(),
            fragment_shader: "./shaders/fragment.glsl".into(),
            params: MaterialParams {
                base_color: [1.0, 1.0, 1.0, 1.0], 
                roughness: 0.5, 
                metallic: 0.0
            },
            textures: MaterialTextures {
                albedo: Some(
                    "./resources/uv_tester.png".into()
                ), 
                normal: None, 
                roughness: None, 
                metallic: None
            }
        }
    }
}