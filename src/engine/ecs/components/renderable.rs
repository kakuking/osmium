use vulkano::{
    buffer::BufferContents, 
};

use crate::engine::{
    scene::{
        asset_manager::Handle, 
        material::Material, 
        mesh::Mesh
    }
};

#[repr(C)]
#[derive(Debug, BufferContents, Clone, Copy)]
pub struct ObjectPushConstants {
    pub model: [[f32; 4]; 4]
}

#[derive(Clone)]
pub struct MeshRenderable {
    pub mesh: Handle<Mesh>,
    pub material: Handle<Material>,
}

impl MeshRenderable {
    pub fn new(
        mesh: Handle<Mesh>,
        material: Handle<Material>
    ) -> Self {
        Self {
            mesh,
            material,
        }
    }
}