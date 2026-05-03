use std::time::{Duration, Instant};

use winit::{
    event::{Event, WindowEvent}, 
    event_loop::{ControlFlow, EventLoop}
};

use crate::engine::{
    config::{
        config::RendererConfig, 
        material::MaterialConfig
    }, 
    renderer::renderer::Renderer, 
    scene::{
        asset_manager::AssetManager, mesh::{
            Mesh, 
            OsmiumVertex
        }, scene::Scene
    }, window::window_manager::WindowManager 
};

pub struct OsmiumEngine {
    pub config: RendererConfig,
    pub renderer: Renderer,
    pub window_manager: WindowManager,
    pub assets: AssetManager,
    pub event_loop: EventLoop<()>
}

impl OsmiumEngine {
    pub fn init() -> Self {
        let mut config = RendererConfig::new();
        config.render_pass.samples = 2;

        let event_loop = EventLoop::new();

        let (scene, mut assets) = Self::create_basic_scene();
        
        let mut window_manager = WindowManager::init(&config.window_config, &event_loop);

        let renderer = Renderer::init(
            &mut window_manager,
            &config,
            scene, 
            &mut assets
        );

        Self {
            config,
            renderer,
            window_manager,
            assets,
            event_loop
        }
    }

    fn create_basic_scene() -> (Scene, AssetManager) {
        let triangles = vec![
            OsmiumVertex { position: [-0.8, -0.5, 0.0], uv: [0.0, 0.0]},
            OsmiumVertex { position: [ -0.3,  0.5, 0.0], uv: [0.0, 1.0] },
            OsmiumVertex { position: [ 0.2, -0.5, 0.0], uv: [1.0, 0.0] },
        ];

        
        let triangles2 = vec![
            OsmiumVertex { position: [-0.2, 0.5, 0.0], uv: [0.0, 1.0] },
            OsmiumVertex { position: [ 0.3, -0.5, 0.0], uv: [1.0, 0.0] },
            OsmiumVertex { position: [ 0.8, 0.5, 0.0], uv: [1.0, 1.0] },
        ];

        let mesh = Mesh::init(triangles, None);
        let mesh2 = Mesh::init(triangles2, None);

        let material_config = MaterialConfig::new();

        let mut scene = Scene::new();

        let mut asset_manager = AssetManager::new();

        let mesh_handle = asset_manager.add_mesh(mesh); //.push(mesh);
        let mesh2_handle = asset_manager.add_mesh(mesh2); //.push(mesh);
        // scene.meshes.push(mesh2);

        let material_handle = asset_manager.add_material_config(material_config); //.material_configs.push(material_config);
        let material2_config = MaterialConfig::new();
        let _ = asset_manager.add_material_config(material2_config); //.material_configs.push(material_config);
        
        scene.add_object(mesh_handle, material_handle);
        scene.add_object(mesh2_handle, material_handle);

        (scene, asset_manager)
    }

    pub unsafe fn run(self) {
        let mut renderer = self.renderer;
        let window_manager = self.window_manager;
        let mut assets = self.assets;
        let event_loop = self.event_loop;
        let config = self.config;

        let target_frame_time = Duration::from_secs_f64(1.0 / config.target_fps as f64);
        let mut last_frame = Instant::now();
        // let mut frame_count = 0;
        // let mut fps_timer = Instant::now();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

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
                    let now = Instant::now();

                    
                    // frame_count += 1;
                    // if now - fps_timer >= std::time::Duration::from_secs(1) {
                    //     println!("FPS: {}", frame_count);
                    //     frame_count = 0;
                    //     fps_timer = now;
                    // }

                    if now - last_frame >= target_frame_time {
                        last_frame = now;
                        window_manager.get_window().request_redraw();
                    }
                }

                Event::RedrawRequested(_) => {
                    renderer.render(
                        &window_manager,
                        &mut assets
                    );
                }

                _ => {}
            }
        });
    }
}