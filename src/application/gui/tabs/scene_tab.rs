use egui_winit_vulkano::egui;

use crate::{
    application::{
        ecs::{
            components::osmium_object::OsmiumObject,
            systems::osmium_object_system::OsmiumObjectSystem,
        },
        gui::editor_context::EditorContext,
    },
    engine::ecs::{coordinator::Coordinator, Entity},
};

pub fn draw_scene_tab(
    ui: &mut egui::Ui,
    editor_context: &mut EditorContext,
) {
    ui.heading("Scene");

    let roots: Vec<Entity> = editor_context
        .coordinator
        .get_system::<OsmiumObjectSystem>()
        .roots
        .iter()
        .copied()
        .collect();

    for entity in roots {
        draw_object_node(ui, editor_context.coordinator, entity);
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