use egui_winit_vulkano::egui;

use crate::application::gui::{
    editor_context::EditorContext,
    tabs::{
        inspector_tab::draw_inspector_tab,
        scene_tab::draw_scene_tab,
        tab::OsmiumTab,
    },
};

pub struct OsmiumTabViewer<'a> {
    pub editor_context: EditorContext<'a>,
}

impl<'a> egui_dock::TabViewer for OsmiumTabViewer<'a> {
    type Tab = OsmiumTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            OsmiumTab::Scene => {
                draw_scene_tab(ui, &mut self.editor_context);
            }

            OsmiumTab::Inspector => {
                draw_inspector_tab(ui, &mut self.editor_context);
            }
        }
    }
}