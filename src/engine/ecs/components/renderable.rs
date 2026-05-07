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
pub struct ColorPushConstants {
    pub model: [f32; 16]
}

#[repr(C)]
#[derive(Debug, BufferContents, Clone, Copy)]
pub struct ShadowPushConstants {
    pub model: [f32; 16],
    pub view_proj: [f32; 16]
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