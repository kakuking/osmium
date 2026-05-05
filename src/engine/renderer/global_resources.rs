use std::sync::Arc;

use vulkano::{
    buffer::{
        BufferContents, 
        BufferUsage, 
        Subbuffer
    }, descriptor_set::{
        PersistentDescriptorSet, 
        WriteDescriptorSet, 
    }, memory::allocator::{
        MemoryTypeFilter, 
    }, pipeline::{
        Pipeline, 
    }
};

use crate::engine::renderer::{buffer_manager::BufferManager, descriptor_manager::DescriptorManager};

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, BufferContents)]
pub struct CameraGpuData {
    pub view: [[f32; 4]; 4],
    pub proj: [[f32; 4]; 4],
    pub view_proj: [[f32; 4]; 4],
    pub camera_pos: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct PointLightGpuData {
    pub position: [f32; 4],
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DirectionalLightGpuData {
    pub direction: [f32; 4],
    pub color: [f32; 4],
}

pub struct RenderGlobals {
    pub camera: CameraGpuData,
    pub point_lights: Vec<PointLightGpuData>,
    pub directional_lights: Vec<DirectionalLightGpuData>,
}

pub struct GlobalResources {
    pub camera_buffers: Vec<Subbuffer<CameraGpuData>>,
    pub descriptor_sets: Vec<Arc<PersistentDescriptorSet>>
}

impl GlobalResources {
    pub fn new(
        buffer_manager: &BufferManager,
        descriptor_manager: &DescriptorManager,
        pipeline: Arc<dyn Pipeline>,
        frames_in_flight: usize,
    ) -> Self {
        let mut camera_buffers = Vec::new();
        let mut descriptor_sets = Vec::new();

        for _ in 0..frames_in_flight {
            let camera_buffer = buffer_manager.create_buffer_from_data(
                CameraGpuData::default(), 
                Some(BufferUsage::UNIFORM_BUFFER), 
                Some(
                    MemoryTypeFilter::PREFER_HOST | 
                    MemoryTypeFilter::HOST_SEQUENTIAL_WRITE
                )
            );

            let mut writes: Vec<WriteDescriptorSet> = Vec::new();

            descriptor_manager.add_buffer(
                &mut writes, 
                0, 
                camera_buffer.clone()
            );

            let descriptor_set = descriptor_manager.create_set(
                pipeline.clone(), 
                0, 
                writes
            );

            camera_buffers.push(camera_buffer);
            descriptor_sets.push(descriptor_set);
        }

        Self {
            camera_buffers,
            descriptor_sets,
        }
    }

    pub fn update(&self, frame_i: usize, globals: &RenderGlobals) {
        let mut camera_write = self.camera_buffers[frame_i].write().unwrap();
        *camera_write = globals.camera;
    }

    pub fn descriptor_set(&self, frame_i: usize) -> Arc<PersistentDescriptorSet> {
        self.descriptor_sets[frame_i].clone()
    }
}