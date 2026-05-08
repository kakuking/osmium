use std::time::{Duration, Instant};

use nalgebra::{Unit, UnitQuaternion, Vector3};
use winit::{
    event::{
        DeviceEvent, ElementState, 
        Event, KeyboardInput, 
        MouseButton, VirtualKeyCode, 
        WindowEvent
    }, 
    event_loop::{
        ControlFlow, EventLoop
    }, 
    window::CursorGrabMode,
};

use crate::engine::{
    config::{
        camera_config::CameraConfig, 
        light_config::LightConfig, 
        material_config::MaterialConfig, 
        mesh_config::MeshConfig, 
        renderer_config::RendererConfig
    }, ecs::{
        components::{
            camera::Camera, 
            default_controller::DefaultController, 
            default_first_person_controller::FirstPersonController, 
            light::Light, physics::{
                PhysicsBody, PhysicsBodyConfig, 
                PhysicsBodyType, PhysicsCollider
            }, 
            renderable::MeshRenderable, 
            transform::Transform
        }, 
        coordinator::Coordinator, 
        signature::Signature, 
        systems::{
            camera::CameraSystem, 
            default_controller::DefaultControllerSystem, 
            default_first_person_controller::FirstPersonControllerSystem, 
            light::LightSystem, 
            physics::PhysicsSystem, 
            render::RenderSystem
        }
    }, 
    renderer::renderer::Renderer, 
    scene::{
        asset_manager::AssetManager, 
        mesh::Mesh, 
    }, 
    window::{
        event_manager::EngineEvent, 
        window_manager::WindowManager
    } 
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
        coordinator.register_component::<DefaultController>();
        coordinator.register_component::<FirstPersonController>();
        coordinator.register_component::<Transform>();
        coordinator.register_component::<PhysicsBodyConfig>();
        coordinator.register_component::<PhysicsBody>();
        coordinator.register_component::<PhysicsCollider>();
        coordinator.register_component::<Camera>();
        coordinator.register_component::<Light>();

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
            coordinator.register_system::<DefaultControllerSystem>();

            let mut signature = Signature::new();
            signature.set(
                coordinator.get_component_type::<Transform>() as usize, 
                true
            );
            signature.set(
                coordinator.get_component_type::<DefaultController>() as usize, 
                true
            );
            coordinator.set_system_signature::<DefaultControllerSystem>(signature);
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

        {
            coordinator.register_system::<LightSystem>();

            let mut signature = Signature::new();
            signature.set(
                coordinator.get_component_type::<Transform>() as usize, 
                true
            );
            signature.set(
                coordinator.get_component_type::<Light>() as usize, 
                true
            );
            coordinator.set_system_signature::<LightSystem>(signature);
        }

        {
            coordinator.register_system::<FirstPersonControllerSystem>();

            let mut signature = Signature::new();
            signature.set(
                coordinator.get_component_type::<Transform>() as usize, 
                true
            );
            signature.set(
                coordinator.get_component_type::<Camera>() as usize, 
                true
            );
            signature.set(
                coordinator.get_component_type::<FirstPersonController>() as usize,
                true
            );
            coordinator.set_system_signature::<FirstPersonControllerSystem>(signature);
        }

        let mut assets = Self::create_basic_scene(&mut coordinator);

        coordinator.initialize_systems();

        let mut window_manager = WindowManager::init(
            &config.window_config, 
            &event_loop
        );

        let renderer = unsafe {
            Renderer::init(
                &mut window_manager,
                &config,
                &mut assets
            )
        };

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

        // Giant and Menacing cube in the background
        {
            let mut mesh_config = MeshConfig::new();
            mesh_config.filepath = "./resources/Cube.obj".into();

            let mesh = Mesh::init(
                &mesh_config
            );

            let physics_body_config = PhysicsBodyConfig::from_mesh(
                &mesh,
                PhysicsBodyType::Dynamic
            );

            let mesh_handle = asset_manager.add_mesh(mesh);
            
            let mut transform = Transform::new();
            transform.position.z = -2.0;
            transform.position.y = 2.0;

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
                transform
            );

            coordinator.add_component(
                entity,
                physics_body_config
            );
        }
        
        // Plane below giant and menacing cube
        {
            let mut mesh_config = MeshConfig::new();
            mesh_config.filepath = "./resources/Plane.obj".into();

            let mesh = Mesh::init(
                &mesh_config
            );

            let physics_body_config = PhysicsBodyConfig::from_mesh(
                &mesh,
                PhysicsBodyType::Dynamic
            );

            let mesh_handle = asset_manager.add_mesh(mesh);
            
            let mut transform = Transform::new();
            transform.position.z = -2.0;
            transform.position.y = -1.0;

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
                transform
            );

            coordinator.add_component(
                entity, 
                physics_body_config
            );
        }
        
        // camera
        {
            let entity = coordinator.create_entity();

            let mut transform = Transform::new();
            transform.position.z = 2.0;

            coordinator.add_component(
                entity, 
                transform
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
                FirstPersonController::new()
            );
        }

        // Directional Light
        {
            let entity = coordinator.create_entity();

            let mut transform = Transform::new();

            transform.position = Vector3::new(2.0, 2.0, 2.0);
            transform.rotation = UnitQuaternion::from_axis_angle(
                &Unit::new_normalize(Vector3::x()),
                -45.0f32.to_radians()
            ) *
            UnitQuaternion::from_axis_angle(
                &Unit::new_normalize(Vector3::y()),
                45.0f32.to_radians()
            );

            coordinator.add_component(
                entity, 
                transform
            );

            coordinator.add_component(
                entity,
                Light::new(
                    LightConfig::new(true)
                )
            )
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
        let mut frame_count = 0;
        let mut fps_timer = Instant::now();

        // Since its V-sync by default, don't need to limit the frame-rate

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta },
                    ..
                } => {
                    if coordinator.events().mouse_captured() {
                        coordinator
                            .events_mut()
                            .add_mouse_delta(delta.0, delta.1);
                    }
                }
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
                            if key == VirtualKeyCode::Escape && state == ElementState::Pressed {
                                window_manager.get_window().set_cursor_visible(true);

                                let _ = window_manager
                                    .get_window()
                                    .set_cursor_grab(CursorGrabMode::None);

                                coordinator.events_mut().set_mouse_captured(false);
                            }

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
                            if button == MouseButton::Left && state == ElementState::Pressed {
                                window_manager.get_window().set_cursor_visible(false);

                                let _ = window_manager
                                    .get_window()
                                    .set_cursor_grab(CursorGrabMode::Locked)
                                    .or_else(|_| {
                                        window_manager
                                            .get_window()
                                            .set_cursor_grab(CursorGrabMode::Confined)
                                    });

                                coordinator.events_mut().set_mouse_captured(true);
                            }

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
                    println!("Shutting down!");
                }

                Event::MainEventsCleared => {
                    let now = Instant::now();
                    
                    frame_count += 1;
                    if now - fps_timer >= std::time::Duration::from_secs(1) {
                        println!("FPS: {}", frame_count);
                        frame_count = 0;
                        fps_timer = now;
                    }

                    window_manager.get_window().request_redraw();
                }

                Event::RedrawRequested(_) => {
                    let dt = target_frame_time.as_secs_f32();

                    coordinator
                        .update_systems(dt);

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