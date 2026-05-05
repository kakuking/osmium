use std::{fmt::Debug, sync::Arc};

use vulkano::{
    buffer::{
        BufferContents, 
        Subbuffer
    }, 
    descriptor_set::{
        PersistentDescriptorSet, 
        WriteDescriptorSet, 
        allocator::StandardDescriptorSetAllocator
    }, 
    device::Device, 
    image::{
        sampler::Sampler, 
        view::ImageView
    }, 
    pipeline::Pipeline
};

pub struct DescriptorManager {
    pub allocator: StandardDescriptorSetAllocator,
}

impl DescriptorManager {
    pub fn new(device: Arc<Device>) -> Self {
        let allocator = 
            StandardDescriptorSetAllocator::new(
                device.clone(), 
                Default::default()
            )
        ;

        Self {
            allocator,
        }
    }

    pub fn create_set(
        &self, 
        pipeline: Arc<dyn Pipeline>,
        set_index: usize,
        writes: Vec<WriteDescriptorSet>,
    ) -> Arc<PersistentDescriptorSet> {
        let pipeline_layout = pipeline.layout();

        let ds_layouts = pipeline_layout.set_layouts();
        let ds_layout = ds_layouts.get(set_index).unwrap();

        PersistentDescriptorSet::new(
            &self.allocator,
            ds_layout.clone(),
            writes,
            []
        ).unwrap()
    }

    pub fn add_buffer<T>(&self, 
        writes: &mut Vec<WriteDescriptorSet>,
        binding: u32, 
        buffer: Subbuffer<T>
    ) 
    where 
        T: BufferContents + Debug + ?Sized,
    {
        writes.push(
            WriteDescriptorSet::buffer(binding, buffer)
        );
    }

    pub fn add_image_view_sampler(
        &self, 
        writes: &mut Vec<WriteDescriptorSet>,
        binding: u32,
        image_view: Arc<ImageView>,
        sampler: Arc<Sampler>) 
    {
        writes.push(
            WriteDescriptorSet::image_view_sampler(
                binding, 
                image_view, 
                sampler
            )
        )
    }
}