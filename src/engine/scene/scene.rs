use std::sync::Arc;

use vulkano::{
    buffer::Subbuffer, 
    command_buffer::allocator::StandardCommandBufferAllocator, 
    descriptor_set::PersistentDescriptorSet, 
    device::{
        Device, Queue
    }, image::SampleCount, 
    memory::allocator::MemoryAllocator, 
    pipeline::{
        GraphicsPipeline, 
        graphics::viewport::Viewport
    }, render_pass::RenderPass
};

use crate::engine::{
    config::material::MaterialConfig, 
    renderer::{
        buffer_manager::BufferManager, 
        descriptor_manager::DescriptorManager, 
        image_manager::ImageManager, 
        shader_manager::ShaderManager
    }, scene::{
        material::Material, 
        mesh::{
            Mesh, 
            OsmiumVertex
        }
    }
};

pub struct RenderItem {
    vertex_buffer: Subbuffer<[OsmiumVertex]>,
    index_buffer: Option<Subbuffer<[u32]>>,
    num_indices: u32,
    num_vertices: u32,
    material: Material
}

impl RenderItem {
    pub fn get_vertex_buffer(&self) -> Subbuffer<[OsmiumVertex]> {
        self.vertex_buffer.clone()
    }

    pub fn get_index_buffer(&self) -> Option<Subbuffer<[u32]>> {
        self.index_buffer.clone()
    }

    pub fn get_num_indices(&self) -> u32 { 
        self.num_indices
    }

    pub fn get_num_vertices(&self) -> u32 { 
        self.num_vertices
    }

    pub fn get_pipeline(&self) -> Arc<GraphicsPipeline> {
        self.material.get_pipeline()
    }

    pub fn get_descriptor_set(&self) -> Arc<PersistentDescriptorSet> {
        self.material.get_descriptor_set()
    }
}

pub struct Scene {
    pub meshes: Vec<Mesh>,
    pub material_configs: Vec<MaterialConfig>,
    materials: Vec<Material>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            material_configs: Vec::new(),
            materials: Vec::new()
        }
    }

    pub fn initiallize_buffers(&mut self, buffer_manager: &BufferManager) {
        for mesh in &mut self.meshes {
            mesh.create_buffers(buffer_manager);
        }
    }

    pub fn create_materials(&mut self, 
        shader_manager: &ShaderManager,
        image_manager: &ImageManager,
        buffer_manager: &BufferManager,
        command_buffer_allocator: &StandardCommandBufferAllocator,
        queue: Arc<Queue>,
        memory_allocator: Arc<dyn MemoryAllocator>,
    ) {
        for config in &self.material_configs {
            self.materials.push(
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

    pub fn create_pipelines(
        &mut self,
        device: Arc<Device>,
        render_pass: Arc<RenderPass>, 
        viewport: Viewport,
        samples: SampleCount,
        enable_depth: bool,
        descriptor_manager: &DescriptorManager,
    ) {
        for material in &mut self.materials {
            material.recreate_pipeline(
                device.clone(), 
                render_pass.clone(), 
                viewport.clone(), 
                samples, 
                enable_depth,
                descriptor_manager
            );
        }
    }

    pub fn get_render_items(&self) -> Vec<RenderItem> {
        self.meshes
            .iter().map(
                |m| {
                    RenderItem {
                        vertex_buffer: m.get_vertex_buffer(),
                        index_buffer: m.index_buffer.clone(),
                        num_indices: m.num_indices,
                        num_vertices: m.num_vertices,
                        material: self.materials[0].clone()
                    }
                }
            )
            .collect()
    }
}