use crate::engine::{
    scene::{
        asset_manager::{AssetManager, Handle}, material::Material, mesh::{
            Mesh
        }
    }
};

pub struct RenderItem {
    mesh: Handle<Mesh>,
    material: Handle<Material>
}

impl RenderItem {
    pub fn get_mesh<'a>(&self, assets: &'a AssetManager) -> &'a Mesh {
        assets.meshes.get(self.mesh)
    }

    pub fn get_material<'a>(&self, assets: &'a AssetManager) -> &'a Material {
        assets.materials.get(self.material)
    }
}

pub struct SceneObject {
    pub mesh: Handle<Mesh>,
    pub material: Handle<Material>
}

pub struct Scene {
    pub objects: Vec<SceneObject>
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(
        &mut self,
        mesh: Handle<Mesh>,
        material: Handle<Material>,
    ) {
        self.objects.push(SceneObject {
            mesh,
            material,
        });
    }

    pub fn get_render_items(&self) -> Vec<RenderItem> {
        self.objects
            .iter()
            .map(|object| RenderItem {
                mesh: object.mesh,
                material: object.material,
            })
            .collect()
    }
}