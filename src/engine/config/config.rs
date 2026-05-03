use serde::Deserialize;
use vulkano::{
    device::physical::PhysicalDeviceType, 
};

use crate::engine::config::window::WindowConfig;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GpuTypeConfig {
    DiscreteGpu,
    IntegratedGpu,
    VirtualGpu,
    Cpu,
}

impl From<GpuTypeConfig> for PhysicalDeviceType {
    fn from(value: GpuTypeConfig) -> Self {
        match value {
            GpuTypeConfig::DiscreteGpu => PhysicalDeviceType::DiscreteGpu,
            GpuTypeConfig::IntegratedGpu => PhysicalDeviceType::IntegratedGpu,
            GpuTypeConfig::VirtualGpu => PhysicalDeviceType::VirtualGpu,
            GpuTypeConfig::Cpu => PhysicalDeviceType::Cpu,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct RendererConfig {
    pub enable_validation: bool,
    pub enable_depth: bool,

    gpu_priority: Vec<GpuTypeConfig>,

    pub render_pass: RenderPassConfig,
    pub window_config: WindowConfig,
    pub target_fps: u32,
    pub print_fps: bool
}

fn default_gpu_priority() -> Vec<GpuTypeConfig> {
    vec![
        GpuTypeConfig::DiscreteGpu,
        GpuTypeConfig::IntegratedGpu,
        GpuTypeConfig::VirtualGpu,
        GpuTypeConfig::Cpu,
    ]
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            enable_validation: false,
            enable_depth: true,
            gpu_priority: default_gpu_priority(),
            render_pass: RenderPassConfig::default(),
            window_config: WindowConfig::default(),
            target_fps: 60,
            print_fps: true,
        }
    }
}

impl RendererConfig {
    pub fn new() -> Self {
        let mut gpu_priority = Vec::new();
        gpu_priority.push(GpuTypeConfig::DiscreteGpu);
        gpu_priority.push(GpuTypeConfig::IntegratedGpu);
        gpu_priority.push(GpuTypeConfig::VirtualGpu);
        gpu_priority.push(GpuTypeConfig::Cpu);
        
        Self {
            enable_validation: false,
            gpu_priority,

            render_pass: RenderPassConfig::new(),
            enable_depth: true,
            window_config: WindowConfig::new(),
            target_fps: 60,
            print_fps: true,
        }
    }

    pub fn get_gpu_priority(&self) -> Vec<PhysicalDeviceType> {
        self.gpu_priority
            .clone()
            .into_iter()
            .map(Into::into)
            .collect()
    }
}
