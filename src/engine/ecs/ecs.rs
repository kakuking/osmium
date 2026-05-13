use crate::engine::ecs::{
    components::{
        camera::Camera, 
        default_controller::DefaultController, 
        default_first_person_controller::FirstPersonController, 
        light::Light, 
        physics::{
            PhysicsBody, 
            PhysicsBodyConfig, 
            PhysicsCollider
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
};

pub fn initialize_default_components(
    coordinator: &mut Coordinator
) {
    coordinator.register_component::<MeshRenderable>();
    coordinator.register_component::<DefaultController>();
    coordinator.register_component::<FirstPersonController>();
    coordinator.register_component::<Transform>();
    coordinator.register_component::<PhysicsBodyConfig>();
    coordinator.register_component::<PhysicsBody>();
    coordinator.register_component::<PhysicsCollider>();
    coordinator.register_component::<Camera>();
    coordinator.register_component::<Light>();
}

pub fn initialize_default_systems(
    coordinator: &mut Coordinator
) {
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
}