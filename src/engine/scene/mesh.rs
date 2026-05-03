use vulkano::{
    buffer::{
        BufferContents, 
        BufferUsage, 
        Subbuffer
    }, 
    memory::allocator::MemoryTypeFilter, 
    pipeline::graphics::vertex_input::Vertex
};

use crate::engine::renderer::buffer_manager::BufferManager;

#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct OsmiumVertex {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],

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
            initialized: false
        }
    }

    pub fn create_buffers(
        &mut self,
        buffer_manager: &BufferManager
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
}