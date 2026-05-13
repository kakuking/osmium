use crate::application::app::OsmiumEngine;

pub mod application;
pub mod engine;

fn main() {
    let app = OsmiumEngine::init();
    
    unsafe {
        app.run();
    }
}
