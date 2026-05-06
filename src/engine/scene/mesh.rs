use std::sync::Arc;

use vulkano::{
    buffer::{
        BufferContents, 
        BufferUsage, 
        Subbuffer
    }, descriptor_set::PersistentDescriptorSet, memory::allocator::MemoryTypeFilter, pipeline::{GraphicsPipeline, graphics::vertex_input::Vertex}
};

use crate::engine::{
    renderer::{
        buffer_manager::BufferManager, 
        descriptor_manager::DescriptorManager, 
        image_manager::{
            ImageManager, 
            Texture
        }
    }, 
    scene::asset_manager::{AssetStorage, Handle}
};

#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct OsmiumVertex {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],
    // #[format(R32G32B32_SFLOAT)]
    // pub normal: [f32; 3],
    #[format(R32G32_SFLOAT)]
    pub uv: [f32; 2],
}

pub struct Mesh {
    pub vertex_buffer: Option<Subbuffer<[OsmiumVertex]>>,
    pub index_buffer: Option<Subbuffer<[u32]>>,
    
    vertices: Vec<OsmiumVertex>,
    indices: Option<Vec<u32>>,
    pub num_indices: u32,
    pub num_vertices: u32,

    height_map_texture: Option<Handle<Arc<Texture>>>,
    descriptor_set: Option<Arc<PersistentDescriptorSet>>,

    initialized: bool
}

impl Mesh {
    pub fn init(
        vertices: Vec<OsmiumVertex>,
        indices: Option<Vec<u32>>,
    ) -> Self {
        let num_vertices = vertices.len() as u32;
        let num_indices = match &indices {
            Some(i) => i.len() as u32,
            None => 0
        };

        Self {
            vertex_buffer: None,
            index_buffer: None,

            vertices,
            indices,
            num_indices,
            num_vertices,

            height_map_texture: None,
            descriptor_set: None,
            initialized: false
        }
    }

    pub fn create_gpu_resources(
        &mut self,
        textures: &AssetStorage<Arc<Texture>>,
        buffer_manager: &BufferManager,
        pipeline: Arc<GraphicsPipeline>,
        descriptor_manager: Arc<DescriptorManager>,
        image_manager: &ImageManager,
    ) {
        let vertex_buffer = buffer_manager.create_buffer_from_iter(
            std::mem::take(&mut self.vertices), 
            Some(BufferUsage::VERTEX_BUFFER), 
            Some(MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE)
        );

        let index_buffer = match std::mem::take(&mut self.indices) {
            Some(idx_buffer) => {
                Some (
                    buffer_manager.create_buffer_from_iter(
                        idx_buffer, 
                        Some(BufferUsage::INDEX_BUFFER), 
                        Some(
                            MemoryTypeFilter::PREFER_DEVICE | 
                            MemoryTypeFilter::HOST_SEQUENTIAL_WRITE
                        )
                    )
                )
            },
            _ => { None }
        };

        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = index_buffer;
        self.initialized = true;

        let mut writes = Vec::new();

        let height = self.height_map_texture
            .map(|handle| textures.get(handle).clone())
            .unwrap_or(image_manager.default_textures.black.clone());

        descriptor_manager.add_image_view_sampler(
            &mut writes, 
            0, 
            height.view.clone(), 
            height.sampler.clone()
        );

        self.descriptor_set = Some(
            descriptor_manager.create_set(
                pipeline, 
                2, 
                writes
            )
        );
    }

    pub fn get_num_vertices(&self) -> u32 {
        self.num_vertices
    }

    pub fn get_num_indices(&self) -> u32 {
        self.num_indices
    }

    pub fn get_vertex_buffer(&self) -> Subbuffer<[OsmiumVertex]> {
        match &self.vertex_buffer {
            Some(buf) => buf.clone(),
            None => panic!("Did not create vertex buffers")
        }
    }

    pub fn get_descriptor_set(&self) -> Arc<PersistentDescriptorSet> {
        match &self.descriptor_set {
            Some(ds) => ds.clone(),
            None => panic!("Did not create mesh gpu resources")
        }
    }
}