use std::time::{Duration, Instant};

use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent}, 
    event_loop::{ControlFlow, EventLoop},
};

use crate::engine::{
    config::{
        camera_config::CameraConfig, material_config::MaterialConfig, renderer_config::RendererConfig
    }, ecs::{
        components::{
            camera::Camera, 
            movement_speeds::MovementSpeeds, 
            physics::{
                PhysicsBody, 
                PhysicsBodyConfig, 
                PhysicsBodyType, 
                PhysicsCollider
            }, 
            renderable::MeshRenderable, 
            transform::Transform
        }, coordinator::Coordinator, 
        signature::Signature, 
        systems::{
            camera::CameraSystem, physics::PhysicsSystem, render::RenderSystem, user_controller::UserControllerSystem
        }
    }, 
    renderer::renderer::Renderer, 
    scene::{
        asset_manager::AssetManager, 
        mesh::{
            Mesh, 
            OsmiumVertex
        }, 
    }, 
    window::{event_manager::EngineEvent, window_manager::WindowManager} 
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
        coordinator.register_component::<MovementSpeeds>();
        coordinator.register_component::<Transform>();
        coordinator.register_component::<PhysicsBodyConfig>();
        coordinator.register_component::<PhysicsBody>();
        coordinator.register_component::<PhysicsCollider>();
        coordinator.register_component::<Camera>();

        {
            coordinator.register_system::<PhysicsSystem>();

            let mut signature = Signature::new();
            signature.set(
                coordinator.get_component_type::<Transform>() as usize, 
                true
            );
            signature.set(
                coordinator.get_component_type::<PhysicsBodyConfig>() as usize, 
                true
            );

            coordinator.set_system_signature::<PhysicsSystem>(signature);
        }

        {
            coordinator.register_system::<RenderSystem>();

            let mut signature = Signature::new();
            signature.set(
                coordinator.get_component_type::<MeshRenderable>() as usize,
                true
            );
            signature.set(
                coordinator.get_component_type::<Transform>() as usize,
                true
            );

            coordinator.set_system_signature::<RenderSystem>(signature);
        }

        {
            coordinator.register_system::<UserControllerSystem>();

            let mut signature = Signature::new();
            signature.set(
                coordinator.get_component_type::<Transform>() as usize, 
                true
            );
            signature.set(
                coordinator.get_component_type::<MovementSpeeds>() as usize, 
                true
            );
            coordinator.set_system_signature::<UserControllerSystem>(signature);
        }

        {
            coordinator.register_system::<CameraSystem>();

            let mut signature = Signature::new();
            signature.set(
                coordinator.get_component_type::<Transform>() as usize, 
                true
            );
            signature.set(
                coordinator.get_component_type::<Camera>() as usize, 
                true
            );
            coordinator.set_system_signature::<CameraSystem>(signature);
        }

        let mut assets = Self::create_basic_scene(&mut coordinator);

        coordinator.initialize_systems();

        let mut window_manager = WindowManager::init(
            &config.window_config, 
            &event_loop
        );

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
        let mut asset_manager = AssetManager::new();
        
        let material_config = MaterialConfig::new();
        let material_handle = asset_manager.add_material_config(material_config);

        // static triangle
        {
            let vertices = vec![
                OsmiumVertex {position: [-0.8, -0.5, 0.1], uv: [0.0, 0.0]},
                OsmiumVertex {position: [ -0.3,  0.5, 0.1], uv: [0.0, 1.0]},
                OsmiumVertex {position: [ 0.2, -0.5, 0.1], uv: [1.0, 0.0]},
            ];
            
            let mesh = Mesh::init(
                vertices, 
                None
            );

            let mesh_handle = asset_manager.add_mesh(mesh);

            let entity = coordinator.create_entity();

            coordinator.add_component(
                entity, 
                MeshRenderable::new(
                    mesh_handle, 
                    material_handle
                )
            );

            coordinator.add_component(
                entity, 
                Transform::new()
            );
        }

        // falling rectangle
        {
            let vertices = vec![
                OsmiumVertex {position: [-0.5, -0.5, 0.2], uv: [0.0, 0.0]},
                OsmiumVertex {position: [ 0.5, -0.5, 0.2], uv: [1.0, 0.0]},
                OsmiumVertex {position: [ -0.5,  0.5, 0.2], uv: [0.0, 1.0]},
                OsmiumVertex {position: [ 0.5, -0.5, 0.2], uv: [1.0, 0.0]},
                OsmiumVertex {position: [ -0.5,  0.5, 0.2], uv: [0.0, 1.0]},
                OsmiumVertex {position: [ 0.5,  0.5, 0.2], uv: [1.0, 1.0]},
            ];

            let collider = PhysicsBodyConfig::from_vertices(
                &vertices,
                0.5,
                PhysicsBodyType::Dynamic
            );

            let mesh = Mesh::init(
                vertices, 
                None
            );

            let mesh_handle = asset_manager.add_mesh(mesh);
            
            let entity = coordinator.create_entity();

            coordinator.add_component(
                entity, 
                MeshRenderable::new(
                    mesh_handle, 
                    material_handle
                )
            );

            coordinator.add_component(
                entity, 
                Transform::new()
            );

            coordinator.add_component(
                entity, 
                collider
            );
        }

        // Static floor
        {
            let vertices = vec![
                OsmiumVertex {position: [-0.8, -0.05, 0.2], uv: [0.0, 0.0]},
                OsmiumVertex {position: [ 0.8, -0.05, 0.2], uv: [1.0, 0.0]},
                OsmiumVertex {position: [ -0.8,  0.15, 0.2], uv: [0.0, 1.0]},
                OsmiumVertex {position: [ 0.8, -0.05, 0.2], uv: [1.0, 0.0]},
                OsmiumVertex {position: [ -0.8,  0.15, 0.2], uv: [0.0, 1.0]},
                OsmiumVertex {position: [ 0.8,  0.15, 0.2], uv: [1.0, 1.0]},
            ];

            let collider = PhysicsBodyConfig::from_vertices(
                &vertices,
                0.5,
                PhysicsBodyType::Fixed
            );
            
            let mesh = Mesh::init(
                vertices, 
                None
            );
    
            let mesh_handle = asset_manager.add_mesh(mesh);
    
            let entity = coordinator.create_entity();
            let mut transform = Transform::new();
            transform.position.y = -1.0;
            transform.position.z = 0.1;
    
            coordinator.add_component(
                entity,
                MeshRenderable::new(
                    mesh_handle, 
                    material_handle
                ),
            );
    
            coordinator.add_component(
                entity, 
                transform
            );

            coordinator.add_component(
                entity,
                collider
            );
        }
        
        // camera
        {
            let entity = coordinator.create_entity();

            let mut camera_transform = Transform::new();
            camera_transform.position.z = 2.0;

            coordinator.add_component(
                entity, 
                camera_transform
            );

            coordinator.add_component(
                entity, 
                Camera::new(
                    CameraConfig::new(true),
                    true
                )
            );

            coordinator.add_component(
                entity, 
                MovementSpeeds::new()
            );
        }

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
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }

                        WindowEvent::Resized(size) => {
                            renderer.resize(size.width, size.height);

                            coordinator.send_event(EngineEvent::WindowResized {
                                width: size.width,
                                height: size.height,
                            });
                        }

                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(key),
                                    state,
                                    ..
                                },
                            ..
                        } => {
                            match state {
                                ElementState::Pressed => {
                                    coordinator.send_event(EngineEvent::KeyPressed(key));
                                }

                                ElementState::Released => {
                                    coordinator.send_event(EngineEvent::KeyReleased(key));
                                }
                            }
                        }

                        WindowEvent::MouseInput { state, button, .. } => {
                            match state {
                                ElementState::Pressed => {
                                    coordinator.send_event(EngineEvent::MousePressed(button));
                                }
                                ElementState::Released => {
                                    coordinator.send_event(EngineEvent::MouseReleased(button));
                                }
                            }
                        }

                        WindowEvent::CursorMoved { position, .. } => {
                            coordinator.send_event(EngineEvent::MouseMoved {
                                x: position.x,
                                y: position.y,
                            });
                        }

                        _ => {}
                    }
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

                    // renderer.rebuild_command_buffers(
                    //     &coordinator.get_render_items(), 
                    //     &assets
                    // );

                    let extent = window_manager.get_window().inner_size();
                    let aspect_ratio = extent.width as f32 / extent.height.max(1) as f32;

                    renderer.render(
                        &window_manager,
                        &mut assets,
                        &coordinator.get_render_items(),
                        &coordinator.get_global_resources(aspect_ratio)
                    );

                    coordinator.clear_frame_events();
                }

                _ => {}
            }
        });
    }
}