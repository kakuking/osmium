use std::sync::Arc;

use egui_winit_vulkano::{
    egui,
    Gui, 
    GuiConfig
};
use vulkano::{
    command_buffer::SecondaryAutoCommandBuffer, 
    render_pass::Subpass
};
use winit::{
    event::WindowEvent, 
    event_loop::EventLoop
};

use crate::engine::{renderer::renderer::{Renderer}, window::window_manager::WindowManager};

pub struct OsmiumGUI {
    gui: Gui
}

impl OsmiumGUI {
    pub fn new(
        event_loop: &EventLoop<()>,
        window_manager: &WindowManager,
        renderer: &Renderer
    ) -> Self {
        let gui = Gui::new_with_subpass(
            event_loop, 
            window_manager.get_surface(), 
            renderer.vulkan_context.get_queue(), 
            Subpass::from(
                renderer.render_pass.clone(), 
                1
            ).unwrap(),
            renderer.swapchain_manager.get_image_format(), 
            GuiConfig::default()
        );

        Self {
            gui
        }
    }

    pub fn update(
        &mut self,
        event: &WindowEvent
    ) {
        self.gui.update(event);
    }

    pub fn create_ui(
        &mut self
    ) {
        self.gui.immediate_ui(|gui| {
            let ctx = gui.context();

            egui::Window::new("Osmium Engine")
                .default_width(260.0)
                .show(&ctx, |ui| {
                    ui.heading("Scene");
                    ui.separator();

                    ui.label("Renderer running");
                    ui.label("Camera active");

                    if ui.button("Test button").clicked() {
                        println!("Clicked UI button");
                    }
                });
        });
    }

    pub fn render(
        &mut self,
        framebuffer_dimensions: [u32; 2]
    ) -> Arc<SecondaryAutoCommandBuffer> {
        self.gui.draw_on_subpass_image(framebuffer_dimensions)
    }
}