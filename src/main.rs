use crate::core::app::Application;

pub mod core;

fn main() {
    let mut app = Application::init(true);

    unsafe {
        app.run();
    }
    
    println!("No errors occured!");
}
