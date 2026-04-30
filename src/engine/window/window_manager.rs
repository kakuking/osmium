use std::sync::Arc;

use vulkano::{
    instance::{Instance, InstanceExtensions}, 
    swapchain::Surface
};

use winit::{
    dpi::{PhysicalSize}, event_loop::EventLoop, 
    window::{
        Window, 
        WindowBuilder
    }
};

pub struct WindowManager {
    window: Arc<Window>,
    enabled_extensions: InstanceExtensions,
    surface: Option<Arc<Surface>>,
}

impl WindowManager {
    pub fn init(event_loop: &EventLoop<()>) -> Self {
        let window: Window = WindowBuilder::new()
            .with_title("Osmium")
            .with_inner_size(PhysicalSize::new(
                1536, 1536
            ))
            .build(event_loop)
            .unwrap();

        let enabled_extensions: InstanceExtensions = Surface::required_extensions(
            event_loop
        );

        Self {
            window: Arc::new(window),
            enabled_extensions,
            surface: None,
        }
    }

    pub fn create_surface(&mut self, instance: Arc<Instance>) {
        let surface: Arc<Surface> = Surface::from_window(
            instance,
            self.window.clone()
        ).unwrap();

        self.surface = Some(surface);
    }

    pub fn get_required_extensions(&self) -> InstanceExtensions {
        self.enabled_extensions.clone()
    }

    pub fn get_surface(&self) -> Arc<Surface> {
        match &self.surface {
            Some(s) => s.clone(),
            _ => panic!("No surface created yet!")
        }
    }
    
    pub fn get_window(&self) -> Arc<Window> {
        self.window.clone()
    }
}