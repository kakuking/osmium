#[derive(Clone, PartialEq)]
pub enum OsmiumTab {
    Scene,
    Inspector,
}

impl OsmiumTab {
    pub fn title(&self) -> &'static str {
        match self {
            OsmiumTab::Scene => "Scene",
            OsmiumTab::Inspector => "Inspector",
        }
    }
}