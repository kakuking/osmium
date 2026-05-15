use std::time::{Duration, Instant};

use winit::{
    event::{
        DeviceEvent, ElementState, 
        Event, KeyboardInput, 
        MouseScrollDelta, WindowEvent
    }, 
    event_loop::{
        ControlFlow, EventLoop
    }, 
};

use crate::{
    application::{
        ecs::{
            components::{osmium_camera::OsmiumCameraController, osmium_object::OsmiumObject}, 
            systems::{osmium_camera_controller_system::OsmiumCameraControllerSystem, osmium_object_system::OsmiumObjectSystem}
        }, 
        gui::gui::OsmiumGUI
    }, 
    engine::{
        config::{
            camera_config::CameraConfig, 
            material_config::MaterialConfig, 
            mesh_config::MeshConfig, 
            renderer_config::RendererConfig
        }, ecs::{
            components::{
                camera::Camera, 
                renderable::MeshRenderable, 
                transform::Transform
            }, 
            coordinator::Coordinator, 
            ecs::{
                initialize_default_components, 
                initialize_default_systems
            }, signature::Signature,
        }, 
        renderer::renderer::Renderer, 
        scene::{asset_manager::AssetManager, mesh::Mesh}, 
        window::{
            event_manager::EngineEvent, 
            window_manager::WindowManager
        } 
    }
};

pub struct OsmiumEngine {
    pub config: RendererConfig,
    pub renderer: Renderer,
    pub asset_manager: AssetManager,
    pub coordinator: Coordinator,
    pub event_loop: EventLoop<()>,
    pub gui: OsmiumGUI,
}

impl OsmiumEngine {
    pub fn init() -> Self {
        let mut config = RendererConfig::new();
        config.render_pass.samples = 2;

        let event_loop = EventLoop::new();

        let window_manager = WindowManager::init(
            &config.window_config, 
            &event_loop
        );

        let mut coordinator: Coordinator = Coordinator::new(
            window_manager
        );

        initialize_default_components(&mut coordinator);
        initialize_default_systems(&mut coordinator);

        coordinator.register_component::<OsmiumCameraController>();
        coordinator.register_component::<OsmiumObject>();

        {
            coordinator.register_system::<OsmiumCameraControllerSystem>();

            let mut signature = Signature::new();
            signature.set(
                coordinator.get_component_type::<Transform>() as usize, 
                true
            );
            signature.set(
                coordinator.get_component_type::<OsmiumCameraController>() as usize, 
                true
            );

            coordinator.set_system_signature::<OsmiumCameraControllerSystem>(signature);
        }

        {
            coordinator.register_system::<OsmiumObjectSystem>();

            let mut signature = Signature::new();

            signature.set(
                coordinator.get_component_type::<OsmiumObject>() as usize, 
                true
            );

            coordinator.set_system_signature::<OsmiumObjectSystem>(signature);
        }


        let mut asset_manager = Self::initialize_scene(&mut coordinator);

        coordinator.initialize_systems();

        let renderer = unsafe {
            Renderer::init(
                &mut coordinator.window_manager,
                &config,
                &mut asset_manager
            )
        };

        let gui = OsmiumGUI::new(
            &event_loop, 
            &coordinator.window_manager, 
            &renderer
        );

        Self {
            config,
            renderer,
            asset_manager,
            coordinator,
            event_loop,
            gui
        }
    }

    fn initialize_scene(coordinator: &mut Coordinator) -> AssetManager {
        let mut asset_manager = AssetManager::new();

        let material_config = MaterialConfig::new();
        let material_handle = asset_manager.add_material_config(material_config);

        let root_entity = {
            let entity = coordinator.create_entity();

            coordinator.add_component(
                entity, 
                OsmiumObject::new_isolated(
                    "root", 
                    entity
                )
            );

            entity
        };
        
        {
            let mut mesh_config = MeshConfig::new();
            mesh_config.filepath = "./resources/Plane.obj".into();

            let mesh = Mesh::init(
                &mesh_config
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

            let entity_object = OsmiumObject::init(
                "plane mesh", 
                entity, 
                root_entity, 
                coordinator
            );

            coordinator.add_component(
                entity, 
                entity_object
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
                OsmiumCameraController::new()
            );

            let entity_object = OsmiumObject::init(
                "camera", 
                entity, 
                root_entity, 
                coordinator
            );

            coordinator.add_component(
                entity, 
                entity_object
            );
        }

        asset_manager
    }

    pub unsafe fn run(self) {
        let mut renderer = self.renderer;
        let mut asset_manager = self.asset_manager;
        let mut coordinator = self.coordinator;
        let mut gui = self.gui;
        let event_loop = self.event_loop;
        let config = self.config;

        let mut last_frame_time = Instant::now();

        let mut fps_timer = Instant::now();
        let mut frame_count: u32 = 0;

        event_loop.run(move |
            event, 
            _event_loop_window_target, 
            control_flow| 
            {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta },
                    ..
                } => {
                    coordinator
                        .events_mut()
                        .add_mouse_delta(delta.0, delta.1);
                }
                Event::WindowEvent { event, .. } => {
                    gui.update(&event);

                    let pointer_captured = gui.wants_pointer_input();
                    let keyboard_captured = gui.wants_keyboard_input();

                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }

                        WindowEvent::Resized(size) => {
                            renderer.resize(size.width, size.height);

                            coordinator.send_event(
                                EngineEvent::WindowResized {
                                    width: size.width,
                                    height: size.height,
                                }
                            );
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
                            if !keyboard_captured {
                                match state {
                                    ElementState::Pressed => {
                                        coordinator.send_event(EngineEvent::KeyPressed(key));
                                    }
                                    ElementState::Released => {
                                        coordinator.send_event(EngineEvent::KeyReleased(key));
                                    }
                                }
                            }
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            if !pointer_captured {
                                match state {
                                    ElementState::Pressed => {
                                        coordinator.send_event(EngineEvent::MousePressed(button));
                                    }
                                    ElementState::Released => {
                                        coordinator.send_event(EngineEvent::MouseReleased(button));
                                    }
                                }
                            }
                        }

                        WindowEvent::MouseWheel { delta, .. } => {
                            let scroll = match delta {
                                MouseScrollDelta::LineDelta(_, y) => y as f64,
                                MouseScrollDelta::PixelDelta(pos) => pos.y,
                            };

                            coordinator.events_mut().add_scroll_delta(scroll);
                        }

                        WindowEvent::CursorMoved { position, .. } => {
                            if !pointer_captured {
                                coordinator.send_event(EngineEvent::MouseMoved 
                                    {
                                        x: position.x,
                                        y: position.y,
                                    }
                                );
                            }
                        }

                        _ => {}
                    }
                }

                Event::LoopDestroyed => {
                    println!("Shutting down!");
                }

                Event::MainEventsCleared => {
                    coordinator.window_manager.request_redraw();
                }

                Event::RedrawRequested(_) => {
                    let now = Instant::now();

                    let delta_time = now
                        .duration_since(last_frame_time)
                        .as_secs_f32();

                    last_frame_time = now;

                    frame_count += 1;

                    if fps_timer.elapsed() >= Duration::from_secs(1) && config.print_fps {
                        let elapsed = fps_timer.elapsed().as_secs_f32();
                        let fps = frame_count as f32 / elapsed;

                        println!("FPS: {:.1}", fps);

                        frame_count = 0;
                        fps_timer = now;
                    }

                    coordinator.update_systems(delta_time);

                    let extent = coordinator.window_manager.get_inner_size();
                    let aspect_ratio = extent.width as f32 / extent.height.max(1) as f32;

                    let global_resources = coordinator.get_global_resources(aspect_ratio);
                    let render_items = coordinator.get_render_items();

                    gui.generate_ui(&mut coordinator);

                    renderer.render(
                        &coordinator.window_manager,
                        &mut asset_manager,
                        &render_items,
                        global_resources,
                        Some(&mut gui)
                    );

                    coordinator.clear_frame_events();
                }

                _ => {}
            }
        });
    }
}