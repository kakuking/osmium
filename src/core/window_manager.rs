use std::sync::Arc;

use vulkano::{instance::{Instance, InstanceExtensions}, swapchain::Surface};
use winit::{event_loop::EventLoop, window::{Window, WindowBuilder}};

pub struct WindowManager {
    window: Arc<Window>,
    required_extensions: InstanceExtensions,
    surface: Option<Arc<Surface>>,
}

impl WindowManager {
    pub fn init(event_loop: &EventLoop<()>) -> Self {
        let window: Arc<Window> = Arc::new(
            WindowBuilder::new()
                .build(event_loop)
                .unwrap()
        );

        Self {
            window,
            required_extensions: Surface::required_extensions(event_loop),
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
        self.required_extensions.clone()
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