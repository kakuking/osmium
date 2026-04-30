use vulkano::{
    device::physical::PhysicalDeviceType, 
};

pub struct RenderPassConfig {
    pub samples: u32,
    pub clear_color: bool,
    pub clear_depth: bool,
    pub store_color: bool,
    pub depth_enabled: bool,
}

impl RenderPassConfig {
    pub fn new() -> Self {
        Self {
            samples: 1,
            clear_color: true,
            clear_depth: true,
            store_color: true,
            depth_enabled: false,
        }
    }
}

pub struct RendererConfig {
    pub enable_validation: bool,
    pub gpu_priority: Vec<PhysicalDeviceType>,

    pub render_pass: RenderPassConfig,
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
            gpu_priority,

            render_pass: RenderPassConfig::new(),
        }
    }
}
