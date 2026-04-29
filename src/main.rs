use crate::engine::app::OsmiumEngine;

pub mod engine;

fn main() {
    let app = OsmiumEngine::init();
    
    unsafe {
        app.run();
    }
}
