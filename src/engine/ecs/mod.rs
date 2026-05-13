pub type Entity = u32;
pub type ComponentType = u8;

pub const MAX_ENTITIES: usize = 4096;
pub const MAX_COMPONENTS: usize = 32;

pub mod entity_manager;
pub mod signature;
pub mod component_array;
pub mod component_manager;
pub mod system;
pub mod system_manager;
pub mod coordinator;
pub mod components;
pub mod systems;
pub mod world_coordinator;

pub mod ecs;