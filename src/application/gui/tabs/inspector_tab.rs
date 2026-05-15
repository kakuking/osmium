use egui_winit_vulkano::egui;

use crate::application::gui::editor_context::EditorContext;

pub fn draw_inspector_tab(
    ui: &mut egui::Ui,
    _editor_context: &mut EditorContext,
) {
    ui.heading("Inspector");
    ui.label("Nothing selected.");
}