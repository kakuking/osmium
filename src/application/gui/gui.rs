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

use crate::{
    application::ecs::{
        components::osmium_object::OsmiumObject, 
        systems::osmium_object_system::OsmiumObjectSystem
    }, 
    engine::{
        ecs::{Entity, coordinator::Coordinator}, 
        renderer::renderer::Renderer, 
        window::window_manager::WindowManager
    }
};

fn draw_scene_hierarchy(ui: &mut egui::Ui, coordinator: &mut Coordinator) {
    ui.heading("Scene");

    let entities: Vec<_> = coordinator
        .get_system::<OsmiumObjectSystem>()
        .roots
        .iter()
        .copied()
        .collect();

    for entity in entities {
        let object = coordinator.get_component::<OsmiumObject>(entity);

        if object.parent.is_none() {
            draw_object_node(ui, coordinator, entity);
        }
    }
}

fn draw_object_node(
    ui: &mut egui::Ui,
    coordinator: &Coordinator,
    entity: Entity,
) {
    let object = coordinator.get_component::<OsmiumObject>(entity);

    let name = object.name.clone();
    let children = object.children.clone();

    egui::CollapsingHeader::new(name)
        .id_source(entity)
        .default_open(true)
        .show(ui, |ui| {
            for child in children {
                draw_object_node(ui, coordinator, child);
            }
        });
}

#[derive(Clone, PartialEq)]
enum OsmiumTab {
    Scene,
    Inspector,
}

struct OsmiumTabViewer<'a> {
    coordinator: &'a mut Coordinator,
}

impl<'a> egui_dock::TabViewer for OsmiumTabViewer<'a> {
    type Tab = OsmiumTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            OsmiumTab::Scene => "Scene".into(),
            OsmiumTab::Inspector => "Inspector".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            OsmiumTab::Scene => {
                draw_scene_hierarchy(ui, self.coordinator);
            }

            OsmiumTab::Inspector => {
                ui.label("Inspector");
            }
        }
    }
}

pub struct OsmiumGUI {
    gui: Gui,
    dock_state: egui_dock::DockState<OsmiumTab>,
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
                OsmiumTab::Scene,
                OsmiumTab::Inspector,
            ]),
        }
    }

    pub fn update(
        &mut self,
        event: &WindowEvent
    ) {
        self.gui.update(event);
    }

    pub fn generate_ui(&mut self, coordinator: &mut Coordinator) {
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
                    let mut tab_viewer = OsmiumTabViewer {
                        coordinator,
                    };

                    egui_dock::DockArea::new(&mut self.dock_state)
                        .show_inside(ui, &mut tab_viewer);
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