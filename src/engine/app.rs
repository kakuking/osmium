use winit::{
    event::{Event, WindowEvent}, 
    event_loop::{ControlFlow, EventLoop}
};

use crate::engine::{
    renderer::{
        config::RendererConfig, 
        renderer::Renderer
    }, 
    window::window_manager::WindowManager, 
};

pub struct OsmiumEngine {
    pub renderer: Renderer,
    pub window_manager: WindowManager,
    pub event_loop: EventLoop<()>
}

impl OsmiumEngine {
    pub fn init() -> Self {
        let mut config = RendererConfig::new();
        config.render_pass.samples = 2;
        config.render_pass.depth_enabled = false;

        let event_loop = EventLoop::new();
        
        let mut window_manager = WindowManager::init(&event_loop);
        let renderer = Renderer::init(
            &mut window_manager, 
            config
        );

        Self {
            renderer,
            window_manager,
            event_loop
        }
    }

    pub unsafe fn run(self) {
        let mut renderer = self.renderer;
        let window_manager = self.window_manager;
        let event_loop = self.event_loop;

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }

                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    renderer.resize(size.width, size.height);
                }

                Event::LoopDestroyed => {
                    println!("No errors occurred!");
                }

                Event::MainEventsCleared => {
                    renderer.render(&window_manager);
                }

                _ => {}
            }
        });
    }
}