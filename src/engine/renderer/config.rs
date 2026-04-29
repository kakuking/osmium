use vulkano::device::physical::PhysicalDeviceType;


pub struct RendererConfig {
    pub enable_validation: bool,
    pub vs_path: String,
    pub fs_path: String,
    pub gpu_priority: Vec<PhysicalDeviceType>,
}

impl RendererConfig {
    pub fn new() -> Self {
        let mut gpu_priority = Vec::new();
        gpu_priority.push(PhysicalDeviceType::DiscreteGpu);
        gpu_priority.push(PhysicalDeviceType::IntegratedGpu);
        gpu_priority.push(PhysicalDeviceType::VirtualGpu);
        gpu_priority.push(PhysicalDeviceType::Cpu);
        
        Self {
            enable_validation: false,
            vs_path: "./shaders/vertex.glsl".to_string(),
            fs_path: "./shaders/fragment.glsl".to_string(),
            gpu_priority
        }
    }
}
