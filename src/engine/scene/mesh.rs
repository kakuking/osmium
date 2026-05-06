use std::sync::Arc;

use vulkano::{
    buffer::{
        BufferContents, 
        BufferUsage, 
        Subbuffer
    }, command_buffer::allocator::{
        StandardCommandBufferAllocator
    }, 
    descriptor_set::PersistentDescriptorSet, 
    device::Queue, 
    memory::allocator::MemoryTypeFilter, 
    pipeline::{
        GraphicsPipeline, 
        graphics::vertex_input::Vertex
    }
};

use crate::engine::{
    config::mesh_config::MeshConfig, renderer::{
        buffer_manager::BufferManager, 
        descriptor_manager::DescriptorManager, 
        image_manager::{
            ImageManager, 
            Texture
        }
    }
};

#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct OsmiumVertex {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],
    #[format(R32G32B32_SFLOAT)]
    pub normal: [f32; 3],
    #[format(R32G32_SFLOAT)]
    pub uv: [f32; 2],
}

impl OsmiumVertex {
    pub fn init(
        position: [f32; 3], 
        normal: [f32; 3], 
        uv: [f32; 2]
    ) -> Self {
        Self {
            position,
            normal,
            uv
        }
    }

    pub fn init_pos_uv(
        position: [f32; 3], 
        uv: [f32; 2]
    ) -> Self {
        Self {
            position,
            normal: [0.0, 0.0, 1.0],
            uv
        }
    }

    pub fn init_pos(position: [f32; 3]) -> Self {
        Self {
            position,
            normal: [0.0, 0.0, 1.0],
            uv: [0.0, 0.0]
        }
    }
}

pub struct Mesh {
    pub vertex_buffer: Option<Subbuffer<[OsmiumVertex]>>,
    pub index_buffer: Option<Subbuffer<[u32]>>,
    
    vertices: Vec<OsmiumVertex>,
    indices: Option<Vec<u32>>,
    pub num_indices: u32,
    pub num_vertices: u32,

    height_map_texture: Option<Arc<Texture>>,   // not through the asset manager to keep the mesh compact
    descriptor_set: Option<Arc<PersistentDescriptorSet>>,

    config: MeshConfig,
    initialized: bool,
}

impl Mesh {
    pub fn init(
        config: &MeshConfig
    ) -> Self {
        let load_options = tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        };

        let (models, _materials) = tobj::load_obj(
            &config.filepath, 
            &load_options
        )
            .map_err(|e| format!("Failed to load OBJ: {e}"))
            .unwrap();

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut vertex_offset: u32 = 0u32;

        for model in models {
            let mesh = model.mesh;

            for i in 0..mesh.positions.len() / 3 {
                let position = [
                    mesh.positions[i * 3],
                    mesh.positions[i * 3 + 1],
                    mesh.positions[i * 3 + 2],
                ];

                let normal = if !mesh.normals.is_empty() {
                    [
                        mesh.normals[i * 3],
                        mesh.normals[i * 3 + 1],
                        mesh.normals[i * 3 + 2],
                    ]
                } else {
                    [0.0, 0.0, 1.0]
                };

                let uv = if !mesh.texcoords.is_empty() {
                    [
                        mesh.texcoords[i * 2],
                        mesh.texcoords[i * 2 + 1],
                    ]
                } else {
                    [0.0, 0.0]
                };

                vertices.push(OsmiumVertex::init(position, normal, uv));
            }

            indices.extend(mesh.indices.iter().map(|i| i + vertex_offset));
            vertex_offset = vertices.len() as u32;
        }

        let num_vertices = vertices.len() as u32;
        let num_indices = indices.len() as u32;

        Self {
            vertex_buffer: None,
            index_buffer: None,

            vertices,
            indices: Some(indices),
            num_indices,
            num_vertices,

            height_map_texture: None,
            descriptor_set: None,
            initialized: false,
            config: config.clone()
        }
    }

    pub fn init_direct(
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
            initialized: false,

            config: MeshConfig::new()
        }
    }

    pub fn create_gpu_resources(
        &mut self,
        buffer_manager: &BufferManager,
        pipeline: Arc<GraphicsPipeline>,
        descriptor_manager: Arc<DescriptorManager>,
        image_manager: &ImageManager,
        command_buffer_allocator: &StandardCommandBufferAllocator,
        queue: Arc<Queue>,
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

        let height = match &self.height_map_texture {
            Some(texture) => texture.clone(),
            None => {
                match &self.config.heightmap {
                    Some(path) => {
                        let texture = image_manager.load_texture(
                            path, 
                            command_buffer_allocator, 
                            queue.clone()
                        );

                        self.height_map_texture = Some(texture.clone());
                        texture
                    },
                    None => image_manager.default_textures.black.clone()
                }
            }
        };

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