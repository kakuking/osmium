use vulkano::{
    device::physical::PhysicalDeviceType, 
};
use winit::{
    dpi::PhysicalSize, 
    window::WindowButtons
};


pub struct WindowConfig {
    pub title: String,
    pub size: PhysicalSize<u32>,
    pub fullscreen: bool,
    pub active: bool,
    pub decorations: bool,
    pub enabled_buttons: WindowButtons,
    pub resizable: bool,
    pub window_icon_path: Option<String>,
    pub taskbar_icon_path: Option<String>,
}

impl WindowConfig {
    pub fn new() -> Self {
        Self {
            title: "Osmium".to_string(),
            size: PhysicalSize::new(1024, 1024),
            fullscreen: false,
            active: true,
            decorations: true,
            enabled_buttons: WindowButtons::CLOSE | WindowButtons::MINIMIZE,
            resizable: true,
            window_icon_path: Some("./resources/osmium.ico".to_string()),
            taskbar_icon_path: Some("./resources/osmium_tb.ico".to_string())
        }
    }
}

pub struct RenderPassConfig {
    pub samples: u32,
    pub clear_color: bool,
    pub clear_depth: bool,
    pub store_color: bool,
}

impl RenderPassConfig {
    pub fn new() -> Self {
        Self {
            samples: 1,
            clear_color: true,
            clear_depth: true,
            store_color: true,
        }
    }
}

pub struct RendererConfig {
    pub enable_validation: bool,
    pub depth_enabled: bool,
    pub gpu_priority: Vec<PhysicalDeviceType>,

    pub render_pass: RenderPassConfig,
    pub window_config: WindowConfig
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
            depth_enabled: true,
            window_config: WindowConfig::new()
        }
    }
}
