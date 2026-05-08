use std::sync::Arc;

use vulkano::{
    buffer::{
        BufferContents, 
        BufferUsage, 
        Subbuffer
    }, descriptor_set::{
        PersistentDescriptorSet, 
        WriteDescriptorSet, 
    }, image::{
        sampler::Sampler, 
        view::ImageView
    }, 
    memory::allocator::MemoryTypeFilter, 
    pipeline::Pipeline
};

use crate::engine::renderer::{
    buffer_manager::BufferManager, 
    descriptor_manager::DescriptorManager
};

pub const MAX_LIGHTS: usize = 16;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, BufferContents)]
pub struct CameraGpuData {
    pub view: [f32; 16],
    pub proj: [f32; 16],
    pub view_proj: [f32; 16],
    pub camera_pos: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, BufferContents)]
pub struct LightGpuData {
    pub view_proj: [f32; 16],
    pub color: [f32; 3],
    pub _pad: f32
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, BufferContents)]
pub struct LightCountsGpuData {
    pub point_light_count: u32,
    pub directional_light_count: u32,
    pub _pad: [u32; 2],
}

pub struct RenderGlobals {
    pub camera: CameraGpuData,
    pub lights: Vec<LightGpuData>,
}

pub struct GlobalResources {
    pub camera_buffers: Vec<Subbuffer<CameraGpuData>>,
    pub light_buffers: Vec<Subbuffer<[LightGpuData]>>,
    pub light_count_buffers: Vec<Subbuffer<LightCountsGpuData>>,

    pub descriptor_sets: Vec<Arc<PersistentDescriptorSet>>
}

impl GlobalResources {
    pub fn new(
        buffer_manager: &BufferManager,
        descriptor_manager: &DescriptorManager,
        pipeline: Arc<dyn Pipeline>,
        frames_in_flight: usize,
        shadow_view: &Vec<Arc<ImageView>>,
        shadow_sampler: &Vec<Arc<Sampler>>,
    ) -> Self {
        let mut camera_buffers = Vec::new();
        let mut light_buffers = Vec::new();
        let mut light_count_buffers = Vec::new();

        let mut descriptor_sets = Vec::new();

        let shadow_maps: Vec<(Arc<ImageView>, Arc<Sampler>)> = shadow_view
            .iter()
            .cloned()
            .zip(shadow_sampler.iter().cloned())
            .collect();

        for _ in 0..frames_in_flight {
            let camera_buffer: Subbuffer<CameraGpuData> = buffer_manager.create_buffer_from_data(
                CameraGpuData::default(), 
                Some(BufferUsage::UNIFORM_BUFFER), 
                Some(
                    MemoryTypeFilter::PREFER_HOST | 
                    MemoryTypeFilter::HOST_SEQUENTIAL_WRITE
                )
            );

            let light_buffer = buffer_manager.create_buffer_from_iter(
                (0..MAX_LIGHTS).map(|_| LightGpuData::default()),
                Some(BufferUsage::STORAGE_BUFFER),
                Some(
                    MemoryTypeFilter::PREFER_HOST |
                    MemoryTypeFilter::HOST_SEQUENTIAL_WRITE
                ),
            );

            let light_count_buffer = buffer_manager.create_buffer_from_data(
                LightCountsGpuData::default(),
                Some(BufferUsage::UNIFORM_BUFFER),
                Some(
                    MemoryTypeFilter::PREFER_HOST |
                    MemoryTypeFilter::HOST_SEQUENTIAL_WRITE
                ),
            );
            
            let mut writes: Vec<WriteDescriptorSet> = Vec::new();
            
            descriptor_manager.add_buffer(&mut writes, 0, camera_buffer.clone());
            descriptor_manager.add_buffer(&mut writes, 1, light_count_buffer.clone());
            descriptor_manager.add_buffer(&mut writes, 2, light_buffer.clone());
            descriptor_manager.add_image_view_sampler_array(&mut writes, 3, shadow_maps.clone());

            let descriptor_set = descriptor_manager.create_set(
                pipeline.clone(), 
                0, 
                writes
            );

            camera_buffers.push(camera_buffer);
            light_buffers.push(light_buffer);
            light_count_buffers.push(light_count_buffer);

            descriptor_sets.push(descriptor_set);
        }

        Self {
            camera_buffers,
            light_buffers,
            light_count_buffers,

            descriptor_sets,
        }
    }

    pub fn update(
        &self, 
        frame_i: usize, 
        globals: &RenderGlobals
    ) {
        {
            let mut camera_write = self.camera_buffers[frame_i].write().unwrap();
            *camera_write = globals.camera;
        }

        {
            let mut counts = self.light_count_buffers[frame_i].write().unwrap();
            counts.point_light_count = globals.lights.len() as u32;
            counts.directional_light_count = globals.lights.len() as u32;
        }

        {
            let mut point_lights = self.light_buffers[frame_i].write().unwrap();

            for (dst, src) in point_lights
                .iter_mut()
                .zip(globals.lights.iter())
            {
                *dst = *src;
            }
        }
    }

    pub fn descriptor_set(&self, frame_i: usize) -> Arc<PersistentDescriptorSet> {
        self.descriptor_sets[frame_i].clone()
    }
}