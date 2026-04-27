use winit::event_loop::EventLoop;

use crate::core::app::Application;

pub mod core;

fn main() {
    let event_loop = EventLoop::new();
    let mut app = Application::init(
        &event_loop,
        true
    );
    unsafe {
        let mut render_data = app.begin();
        event_loop.run(
            move |event, _, control_flow| {
                app.handle_event(event, control_flow, &mut render_data);
            }
        )
    }    
}
