use std::sync::Arc;

use vulkano::{
    instance::{Instance, InstanceExtensions}, 
    swapchain::Surface
};

use winit::{
    dpi::PhysicalSize, 
    event_loop::EventLoop, 
    platform::windows::{
        IconExtWindows, 
        WindowBuilderExtWindows
    }, window::{
        Icon, 
        Window, 
        WindowBuilder
    }
};

use crate::engine::config::window_config::WindowConfig;

pub struct WindowManager {
    window: Arc<Window>,
    enabled_extensions: InstanceExtensions,
    surface: Option<Arc<Surface>>,
}

impl WindowManager {
    pub fn init(
        config: &WindowConfig, 
        event_loop: &EventLoop<()>
    ) -> Self {
        let window_icon = config.window_icon_path
            .as_ref()
            .and_then(|icon_path| {
                match Icon::from_path(icon_path, Some(PhysicalSize::new(32, 32))) {
                    Ok(icon) => Some(icon),
                    Err(err) => {
                        eprintln!("Failed to load window icon: {err:?}");
                        None
                    }
                }
            });

        let taskbar_icon = config.taskbar_icon_path
            .as_ref()
            .and_then(|icon_path| {
                match Icon::from_path(icon_path, Some(PhysicalSize::new(256, 256))) {
                    Ok(icon) => Some(icon),
                    Err(err) => {
                        eprintln!("Failed to load taskbar icon: {err:?}");
                        None
                    }
                }
            });

        let window: Window = WindowBuilder::new()
        .with_title(&config.title)
            .with_active(config.active)
            .with_decorations(config.decorations)
            .with_enabled_buttons(config.enabled_buttons())
            .with_resizable(config.resizable)
            .with_inner_size(config.size())
            .with_window_icon(window_icon)
            .with_taskbar_icon(taskbar_icon)
            .with_visible(config.visible)
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

    pub fn set_visibility(&self, visibile: bool) {
        self.get_window().set_visible(visibile);
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

    pub fn get_inner_size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}