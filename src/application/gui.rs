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

struct OsmiumTabViewer;

impl egui_dock::TabViewer for OsmiumTabViewer {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.as_str().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content for {tab}"));
    }
}

pub struct OsmiumGUI {
    gui: Gui,
    dock_state: egui_dock::DockState<String>,
    tab_viewer: OsmiumTabViewer,
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
            gui,
            dock_state: egui_dock::DockState::new(vec![
                "Scene".to_string(),
                "Inspector".to_string(),
            ]),
            tab_viewer: OsmiumTabViewer {},
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
            let ctx = &gui.context();

            egui::TopBottomPanel::top("top_bar")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Toolbar");
                    });
                });

            egui::SidePanel::left("left_dock_area")
                .resizable(true)
                .default_width(300.0)
                .show(ctx, |ui| {
                    egui_dock::DockArea::new(&mut self.dock_state)
                        .show_inside(ui, &mut self.tab_viewer);
                });
        });
    }

    pub fn render(
        &mut self,
        framebuffer_dimensions: [u32; 2]
    ) -> Arc<SecondaryAutoCommandBuffer> {
        self.gui.draw_on_subpass_image(framebuffer_dimensions)
    }

    pub fn wants_pointer_input(
        &self
    ) -> bool {
        self.gui.context().wants_pointer_input()
    }

    pub fn wants_keyboard_input(
        &self
    ) -> bool {
        self.gui.context().wants_keyboard_input()
    }
}