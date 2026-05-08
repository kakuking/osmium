use crate::engine::{
    scene::{
        asset_manager::{AssetManager, Handle}, material::Material, mesh::Mesh
    }
};

pub struct RenderItem {
    pub mesh: Handle<Mesh>,
    pub material: Handle<Material>,
    pub model_matrix: [f32; 16]
}

impl RenderItem {
    pub fn get_mesh<'a>(&self, assets: &'a AssetManager) -> &'a Mesh {
        assets.meshes.get(self.mesh)
    }

    pub fn get_material<'a>(&self, assets: &'a AssetManager) -> &'a Material {
        assets.materials.get(self.material)
    }
}