use winit::{
    event::{Event, WindowEvent}, 
    event_loop::{ControlFlow, EventLoop}
};

use crate::engine::{
    renderer::{
        config::RendererConfig, 
        renderer::Renderer
    }, scene::{
        material::MaterialConfig, 
        mesh::{
            Mesh, 
            OsmiumVertex
        }, 
        scene::Scene
    }, 
    window::window_manager::WindowManager 
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

        let event_loop = EventLoop::new();

        let scene = Self::create_basic_scene();
        
        let mut window_manager = WindowManager::init(&config.window_config, &event_loop);

        let renderer = Renderer::init(
            &mut window_manager,
            scene, 
            config
        );

        Self {
            renderer,
            window_manager,
            event_loop
        }
    }

    fn create_basic_scene() -> Scene {
        let triangles = vec![
            OsmiumVertex { position: [-0.8, -0.5, 0.0] },
            OsmiumVertex { position: [ -0.3,  0.5, 0.0] },
            OsmiumVertex { position: [ 0.2, -0.5, 0.0] },
        ];

        let mesh = Mesh::init(triangles, None);

        let triangles2 = vec![
            OsmiumVertex { position: [-0.2, 0.5, 0.0] },
            OsmiumVertex { position: [ 0.3, -0.5, 0.0] },
            OsmiumVertex { position: [ 0.8, 0.5, 0.0] },
        ];

        let mesh2 = Mesh::init(triangles2, None);

        let material_config = MaterialConfig::new();

        let mut scene = Scene::new();
        scene.meshes.push(mesh);
        scene.meshes.push(mesh2);

        scene.material_configs.push(material_config);

        scene
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
                    window_manager.set_visibility(true);
                }

                _ => {}
            }
        });
    }
}