use std::time::{Duration, Instant};

use winit::{
    event::{Event, WindowEvent}, 
    event_loop::{ControlFlow, EventLoop}
};

use crate::engine::{
    config::{
        config::RendererConfig, 
        material::MaterialConfig
    }, ecs::{
        components::{
            gravity::Gravity, renderable::MeshRenderable, rigid_body::RigidBody, transform::Transform
        }, coordinator::Coordinator, 
        signature::Signature, 
        systems::{physics::PhysicsSystem, render::RenderSystem}
    }, 
    renderer::renderer::Renderer, 
    scene::{
        asset_manager::AssetManager, 
        mesh::{
            Mesh, 
            OsmiumVertex
        }, 
    }, 
    window::window_manager::WindowManager 
};

pub struct OsmiumEngine {
    pub config: RendererConfig,
    pub renderer: Renderer,
    pub window_manager: WindowManager,
    pub assets: AssetManager,
    pub coordinator: Coordinator,
    pub event_loop: EventLoop<()>,
}

impl OsmiumEngine {
    pub fn init() -> Self {
        let mut config = RendererConfig::new();
        config.render_pass.samples = 2;

        let event_loop = EventLoop::new();

        let mut coordinator = Coordinator::new();

        coordinator.register_component::<MeshRenderable>();
        coordinator.register_component::<Transform>();
        coordinator.register_component::<Gravity>();
        coordinator.register_component::<RigidBody>();

        let _physics_system = coordinator.register_system::<PhysicsSystem>();
        let _render_system = coordinator.register_system::<RenderSystem>();

        let mut physics_signature = Signature::new();
        physics_signature.set(
            coordinator.get_component_type::<Transform>() as usize, 
            true
        );
        physics_signature.set(
            coordinator.get_component_type::<Gravity>() as usize, 
            true
        );
        physics_signature.set(
            coordinator.get_component_type::<RigidBody>() as usize, 
            true
        );
        coordinator.set_system_signature::<PhysicsSystem>(physics_signature);

        let mut render_signature = Signature::new();
        render_signature.set(
            coordinator.get_component_type::<MeshRenderable>() as usize,
            true
        );
        render_signature.set(
            coordinator.get_component_type::<Transform>() as usize,
            true
        );
        coordinator.set_system_signature::<RenderSystem>(render_signature);


        let mut assets = Self::create_basic_scene(&mut coordinator);

        
        let mut window_manager = WindowManager::init(&config.window_config, &event_loop);

        let renderer = Renderer::init(
            &mut window_manager,
            &config,
            &coordinator.get_render_items(), 
            &mut assets
        );

        Self {
            config,
            renderer,
            window_manager,
            assets,
            coordinator,
            event_loop,
        }
    }

    fn create_basic_scene(coordinator: &mut Coordinator) -> AssetManager {
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
        let material_config = MaterialConfig::new();
        let mesh2 = Mesh::init(triangles2, None);

        let mut asset_manager = AssetManager::new();

        let mesh_handle = asset_manager.add_mesh(mesh);
        let mesh2_handle = asset_manager.add_mesh(mesh2);
        let material_handle = asset_manager.add_material_config(material_config);

        let entity = coordinator.create_entity();

        coordinator.add_component(entity, MeshRenderable::new(mesh_handle, material_handle));
        coordinator.add_component(entity, Transform::new());
        coordinator.add_component(entity, Gravity::new());
        coordinator.add_component(entity, RigidBody::new());

        let entity2 = coordinator.create_entity();

        coordinator.add_component(entity2, MeshRenderable::new(mesh2_handle, material_handle));
        coordinator.add_component(entity2, Transform::new());
        coordinator.add_component(entity2, Gravity::new());
        coordinator.add_component(entity2, RigidBody::new());

        asset_manager
    }

    pub unsafe fn run(self) {
        let mut renderer = self.renderer;
        let window_manager = self.window_manager;
        let mut assets = self.assets;
        let event_loop = self.event_loop;
        let config = self.config;
        let mut coordinator = self.coordinator;

        let target_frame_time = Duration::from_secs_f64(1.0 / config.target_fps as f64);
        let mut last_frame = Instant::now();
        let mut frame_count = 0;
        let mut fps_timer = Instant::now();

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

                    
                    frame_count += 1;
                    if now - fps_timer >= std::time::Duration::from_secs(1) {
                        println!("FPS: {}", frame_count);
                        frame_count = 0;
                        fps_timer = now;
                    }

                    if now - last_frame >= target_frame_time {
                        last_frame = now;
                        window_manager.get_window().request_redraw();
                    }
                }

                Event::RedrawRequested(_) => {
                    let dt = target_frame_time.as_secs_f32();

                    coordinator
                        .update_systems(dt);

                    renderer.rebuild_command_buffers(
                        &coordinator.get_render_items(), 
                        &assets
                    );

                    renderer.render(
                        &window_manager,
                        &mut assets,
                        &coordinator.get_render_items()
                    );
                }

                _ => {}
            }
        });
    }
}