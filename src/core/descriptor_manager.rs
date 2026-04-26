use std::{fmt::Debug, sync::Arc};

use vulkano::{buffer::{BufferContents, Subbuffer}, descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet, allocator::StandardDescriptorSetAllocator}, device::Device, image::view::ImageView, pipeline::Pipeline};


pub struct DescriptorManager {
    pub allocator: StandardDescriptorSetAllocator,
    pub descriptor_set: Option<Arc<PersistentDescriptorSet>>,
    pub writes: Vec<WriteDescriptorSet>,
}

impl DescriptorManager {
    pub fn new(device: Arc<Device>) -> Self {
        let allocator = 
            StandardDescriptorSetAllocator::new(device.clone(), Default::default())
        ;

        Self {
            allocator,
            descriptor_set: None,
            writes: Vec::new()
        }
    }

    pub fn get_descriptor_set(&self) -> Arc<PersistentDescriptorSet> {
        match &self.descriptor_set {
            Some(ds) => ds.clone(),
            None => {
                panic!("Descriptor set not initialized, initializing...");
            }
        }
    }

    pub fn initialize(&mut self, descriptor_set_layout_idx: usize, pipeline: Arc<dyn Pipeline>) {
        let pipeline_layout = pipeline.layout();

        let ds_layouts = pipeline_layout.set_layouts();

        let ds_layout = ds_layouts.get(descriptor_set_layout_idx).unwrap();

        let ds = PersistentDescriptorSet::new(
            &self.allocator,
            ds_layout.clone(),
            self.writes.clone(),
            []
        ).unwrap();

        self.descriptor_set = Some(ds);
    }

    pub fn is_initiaized(&self) -> bool {
        self.descriptor_set.is_some()
    }

    pub fn add_buffer<T>(&mut self, binding: u32, buffer: Subbuffer<T>) 
    where 
        T: BufferContents + Debug + ?Sized,
    {
        self.writes.push(
            WriteDescriptorSet::buffer(binding, buffer)
        );
    }

    pub fn add_image_view(&mut self, binding: u32, view: Arc<ImageView>) {
        self.writes.push(
            WriteDescriptorSet::image_view(binding, view)
        );
    }
}