use std::{
    collections::HashMap, 
    marker::PhantomData, 
    path::PathBuf, 
    sync::Arc
};

use shaderc::ShaderKind;
use vulkano::{
    command_buffer::allocator::StandardCommandBufferAllocator, 
    device::Queue, 
    memory::allocator::MemoryAllocator, shader::ShaderModule
};

use crate::engine::{
    config::material_config::MaterialConfig, 
    renderer::{
        buffer_manager::BufferManager, 
        image_manager::{ImageManager, Texture}, 
        shader_manager::ShaderManager
    }, scene::{
        material::{Material, MaterialAssets}, 
        mesh::Mesh
    }
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShaderKey {
    pub path: PathBuf,
    pub kind: ShaderKindKey,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShaderKindKey {
    Vertex,
    Fragment,
}

impl ShaderKindKey {
    pub fn to_shaderc(&self) -> ShaderKind {
        match self {
            ShaderKindKey::Vertex => ShaderKind::Vertex,
            ShaderKindKey::Fragment => ShaderKind::Fragment,
        }
    }
}

pub struct ShaderStorage {
    assets: Vec<Arc<ShaderModule>>,
    keys: HashMap<
        ShaderKey, 
        Handle<Arc<ShaderModule>>
        >,
}

impl ShaderStorage {
    pub fn new() -> Self {
        Self {
            assets: Vec::new(),
            keys: HashMap::new(),
        }
    }

    pub fn load(
        &mut self,
        path: impl Into<PathBuf>,
        kind: ShaderKindKey,
        shader_manager: &ShaderManager,
    ) -> Handle<Arc<ShaderModule>> {
        let key = ShaderKey {
            path: path.into(),
            kind,
        };

        if let Some(handle) = self.keys.get(&key) {
            return *handle;
        }

        let shader = unsafe {
            shader_manager.create_shader(
                key.path.to_str().unwrap(),
                key.kind.to_shaderc(),
            )
        };

        let handle = Handle::new(self.assets.len());
        self.assets.push(shader);
        self.keys.insert(key, handle);

        handle
    }

    pub fn get(&self, handle: Handle<Arc<ShaderModule>>) -> Arc<ShaderModule> {
        self.assets[handle.id()].clone()
    }
}

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
    pub textures: AssetStorage<Arc<Texture>>,
    pub shaders: ShaderStorage,

    pub material_configs: Vec<MaterialConfig>
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            meshes: AssetStorage::new(),
            materials: AssetStorage::new(),
            textures: AssetStorage::new(),
            shaders: ShaderStorage::new(),

            material_configs: Vec::new(),
        }
    }

    pub fn add_mesh(&mut self, asset: Mesh) -> Handle<Mesh> {
        self.meshes.add(asset)
    }

    pub fn add_material(&mut self, asset: Material) -> Handle<Material> {
        self.materials.add(asset)
    }

    pub fn load_texture(
        &mut self,
        path: impl Into<PathBuf>,
        image_manager: &ImageManager,
        command_buffer_allocator: &StandardCommandBufferAllocator,
        queue: Arc<Queue>,
    ) -> Handle<Arc<Texture>> {
        let path = path.into();

        if let Some(handle) = self.textures.paths.get(&path) {
            return *handle;
        }

        let texture = image_manager.load_texture(
            path.to_str().unwrap(),
            command_buffer_allocator,
            queue,
        );

        self.textures.add_with_path(path, texture)
    }

    pub fn load_shader(
        &mut self,
        path: impl Into<PathBuf>,
        kind: ShaderKindKey,
        shader_manager: &ShaderManager,
    ) -> Handle<Arc<ShaderModule>> {
        self.shaders.load(path, kind, shader_manager)
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
        let start = self.materials.len();

        let configs: Vec<MaterialConfig> = self
            .material_configs
            .iter()
            .skip(start)
            .cloned()
            .collect();

        for config in configs {
            let material_assets = MaterialAssets {
                vertex_shader: self.load_shader(
                    &config.vertex_shader,
                    ShaderKindKey::Vertex,
                    shader_manager,
                ),

                fragment_shader: self.load_shader(
                    &config.fragment_shader,
                    ShaderKindKey::Fragment,
                    shader_manager,
                ),

                albedo_texture: config.textures.albedo.as_ref().map(|path| {
                    self.load_texture(
                        path,
                        image_manager,
                        command_buffer_allocator,
                        queue.clone(),
                    )
                }),

                normal_texture: config.textures.normal.as_ref().map(|path| {
                    self.load_texture(
                        path,
                        image_manager,
                        command_buffer_allocator,
                        queue.clone(),
                    )
                }),

                roughness_texture: config.textures.roughness.as_ref().map(|path| {
                    self.load_texture(
                        path,
                        image_manager,
                        command_buffer_allocator,
                        queue.clone(),
                    )
                }),

                metallic_texture: config.textures.metallic.as_ref().map(|path| {
                    self.load_texture(
                        path,
                        image_manager,
                        command_buffer_allocator,
                        queue.clone(),
                    )
                }),
            };

            let material = Material::init(
                &config,
                material_assets,
                buffer_manager,
                command_buffer_allocator,
                queue.clone(),
                memory_allocator.clone(),
            );

            self.materials.add(material);
        }
    }
}