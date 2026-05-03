use std::sync::Arc;

use vulkano::{
    buffer::{
        BufferContents, 
        BufferUsage, 
        Subbuffer
    }, descriptor_set::{
        PersistentDescriptorSet, 
        WriteDescriptorSet, 
        allocator::StandardDescriptorSetAllocator
    }, memory::allocator::{
        MemoryTypeFilter
    }, pipeline::{
        GraphicsPipeline, 
        Pipeline
    }
};

use crate::engine::{
    ecs::components::transform::Transform, renderer::buffer_manager::BufferManager, scene::{
        asset_manager::Handle, 
        material::Material, 
        mesh::Mesh
    }
};

#[repr(C)]
#[derive(Debug, BufferContents, Clone, Copy)]
pub struct ObjectUniform {
    pub model: [[f32; 4]; 4]
}

#[derive(Clone)]
pub struct MeshRenderable {
    pub mesh: Handle<Mesh>,
    pub material: Handle<Material>,

    pub object_buffer: Option<Subbuffer<ObjectUniform>>,
    pub object_descriptor_set: Option<Arc<PersistentDescriptorSet>>,
}

impl MeshRenderable {
    pub fn new(
        mesh: Handle<Mesh>,
        material: Handle<Material>
    ) -> Self {
        Self {
            mesh,
            material,

            object_buffer: None,
            object_descriptor_set: None
        }
    }

    pub fn update_transform_resources(
        &mut self, 
        transform: &Transform
    ) {
        let buffer = self
            .object_buffer
            .as_ref()
            .expect("Object buffer not created");

        let mut write = buffer.write().unwrap();

        *write = ObjectUniform {
            model: transform.model_matrix(),
        };
    }

    pub fn create_gpu_resources(
        &mut self,
        buffer_manager: &BufferManager,
        descriptor_set_allocator: &StandardDescriptorSetAllocator,
        pipeline: Arc<GraphicsPipeline>,
        transform: &Transform,
    ) {
        let object_buffer = buffer_manager.create_buffer_from_data(
            ObjectUniform {
                model: transform.model_matrix()
                }, 
            Some(BufferUsage::UNIFORM_BUFFER),  
            Some(MemoryTypeFilter::HOST_SEQUENTIAL_WRITE)
        );

        let layout = pipeline
            .layout()
            .set_layouts()
            .get(2)
            .unwrap()
            .clone();

        let descriptor_set = PersistentDescriptorSet::new(
            descriptor_set_allocator,
            layout,
            [
                WriteDescriptorSet::buffer(
                    0, object_buffer.clone()
                )
            ],
            [],
        )
        .unwrap();

        self.object_buffer = Some(object_buffer);
        self.object_descriptor_set = Some(descriptor_set);
    }
}