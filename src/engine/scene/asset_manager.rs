use std::{collections::HashMap, marker::PhantomData, path::PathBuf, sync::Arc};

use vulkano::{command_buffer::allocator::StandardCommandBufferAllocator, device::Queue, memory::allocator::MemoryAllocator};

use crate::engine::{config::material::MaterialConfig, renderer::{buffer_manager::BufferManager, image_manager::ImageManager, shader_manager::ShaderManager}, scene::{material::Material, mesh::Mesh}};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Handle<T> {
    id: usize,
    _marker: PhantomData<T>
}

impl<T> Copy for Handle<T> {}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Handle<T> {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            _marker: PhantomData
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

pub struct AssetStorage<T> {
    assets: Vec<T>,
    paths: HashMap<PathBuf, Handle<T>>
}

impl<T> AssetStorage<T> {
    pub fn new() -> Self {
        Self {
            assets: Vec::new(),
            paths: HashMap::new()
        }
    }

    pub fn add(&mut self, asset: T) -> Handle<T> {
        let handle = Handle::new(self.assets.len());
        self.assets.push(asset);
        handle
    }

    pub fn add_with_path(&mut self, path: PathBuf, asset: T) -> Handle<T> {
        if let Some(handle) = self.paths.get(&path) {
            return *handle;
        }

        let handle = self.add(asset);
        self.paths.insert(path, handle);
        handle
    }

    pub fn get(&self, handle: Handle<T>) -> &T {
        &self.assets[handle.id()]
    }

    pub fn get_mut(&mut self, handle: Handle<T>) -> &mut T {
        &mut self.assets[handle.id()]
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.assets.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.assets.len()
    }
}

pub struct AssetManager {
    pub meshes: AssetStorage<Mesh>,
    pub materials: AssetStorage<Material>,
    pub material_configs: Vec<MaterialConfig>
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            meshes: AssetStorage::new(),
            materials: AssetStorage::new(),
            material_configs: Vec::new()
        }
    }

    pub fn add_mesh(&mut self, asset: Mesh) -> Handle<Mesh> {
        self.meshes.add(asset)
    }

    pub fn add_material(&mut self, asset: Material) -> Handle<Material> {
        self.materials.add(asset)
    }

    pub fn add_material_config(&mut self, asset: MaterialConfig) -> Handle<Material> {
        let handle = Handle::new(self.material_configs.len());
        self.material_configs.push(asset);
        handle
    }

    pub fn create_materials(
        &mut self, 
        shader_manager: &ShaderManager,
        image_manager: &ImageManager,
        buffer_manager: &BufferManager,
        command_buffer_allocator: &StandardCommandBufferAllocator,
        queue: Arc<Queue>,
        memory_allocator: Arc<dyn MemoryAllocator>,
    ) {
        if self.material_configs.len() == self.materials.len() {
            return;
        }

        for config in &self.material_configs {
            self.materials.add(
                Material::init(
                    config,
                    shader_manager,
                    image_manager,
                    buffer_manager,
                    command_buffer_allocator,
                    queue.clone(),
                    memory_allocator.clone(),
                )
            );
        }
    }
}