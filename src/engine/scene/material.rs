use std::sync::Arc;

use vulkano::{
    buffer::{
        BufferContents, 
        BufferUsage, 
        Subbuffer
    }, 
    command_buffer::allocator::StandardCommandBufferAllocator, 
    descriptor_set::PersistentDescriptorSet, 
    device::{
        Device, Queue
    }, 
    image::SampleCount, 
    memory::allocator::{
        MemoryAllocator, MemoryTypeFilter
    }, 
    pipeline::{
        GraphicsPipeline, 
        graphics::viewport::Viewport
    }, 
    render_pass::RenderPass, 
    shader::{ShaderModule}
};

use crate::engine::{
    config::material_config::MaterialConfig, renderer::{
        buffer_manager::BufferManager, 
        descriptor_manager::DescriptorManager, 
        image_manager::{
            DefaultTextures, 
            Texture
        }, 
        pipeline_constructor::PipelineConstructor, 
    }, scene::{
        asset_manager::{
            AssetStorage, 
            Handle, 
            ShaderStorage
        }, 
        mesh::OsmiumVertex
    }
};

#[derive(Debug, Clone, Copy, BufferContents)]
#[repr(C)]
pub struct MaterialUniform {
    pub base_color: [f32; 4],

    // x = roughness
    // y = metallic
    // z/w unused
    pub pbr_params: [f32; 4],

    // x = has_albedo
    // y = has_normal
    // z = has_roughness
    // w = has_metallic
    pub texture_flags: [u32; 4],
}

pub struct MaterialAssets {
    pub vertex_shader: Handle<Arc<ShaderModule>>,
    pub fragment_shader: Handle<Arc<ShaderModule>>,

    pub albedo_texture: Option<Handle<Arc<Texture>>>,
    pub normal_texture: Option<Handle<Arc<Texture>>>,
    pub roughness_texture: Option<Handle<Arc<Texture>>>,
    pub metallic_texture: Option<Handle<Arc<Texture>>>,
}

#[derive(Clone)]
pub struct Material {
    vertex_shader: Handle<Arc<ShaderModule>>,
    fragment_shader: Handle<Arc<ShaderModule>>,

    pub name: String,
    pub uniform: MaterialUniform,
    uniform_buffer: Subbuffer<MaterialUniform>,

    albedo_texture: Option<Handle<Arc<Texture>>>,
    normal_texture: Option<Handle<Arc<Texture>>>,
    roughness_texture: Option<Handle<Arc<Texture>>>,
    metallic_texture: Option<Handle<Arc<Texture>>>,

    pipeline: Option<Arc<GraphicsPipeline>>,
    descriptor_set: Option<Arc<PersistentDescriptorSet>>,

    default_textures: DefaultTextures
}

impl Material {
    pub fn init(
        config: &MaterialConfig,
        material_assets: MaterialAssets,
        buffer_manager: &BufferManager,
        command_buffer_allocator: &StandardCommandBufferAllocator,
        queue: Arc<Queue>,
        memory_allocator: Arc<dyn MemoryAllocator>,
    ) -> Self {
        let uniform = MaterialUniform {
            base_color: config.params.base_color,

            pbr_params: [
                config.params.roughness,
                config.params.metallic,
                0.0,
                0.0,
            ],

            texture_flags: [
                material_assets.albedo_texture.is_some() as u32,
                material_assets.normal_texture.is_some() as u32,
                material_assets.roughness_texture.is_some() as u32,
                material_assets.metallic_texture.is_some() as u32,
            ],
        };

        let uniform_buffer = buffer_manager.create_buffer_from_data(
            uniform,
            Some(BufferUsage::UNIFORM_BUFFER),
            Some(MemoryTypeFilter::HOST_SEQUENTIAL_WRITE),
        );

        let default_textures = DefaultTextures::new(
            memory_allocator,
            command_buffer_allocator,
            queue,
        );

        Self {
            name: config.name.clone(),

            vertex_shader: material_assets.vertex_shader,
            fragment_shader: material_assets.fragment_shader,

            uniform,
            uniform_buffer,

            albedo_texture: material_assets.albedo_texture,
            normal_texture: material_assets.normal_texture,
            roughness_texture: material_assets.roughness_texture,
            metallic_texture: material_assets.metallic_texture,

            pipeline: None,
            descriptor_set: None,

            default_textures,
        }
    }

    pub fn recreate_descriptor_set(
        &mut self,
        textures: &AssetStorage<Arc<Texture>>,
        descriptor_manager: &DescriptorManager,
    ) {
        let pipeline = self.get_pipeline();

        let mut writes = Vec::new();

        descriptor_manager.add_buffer(
            &mut writes,
            0,
            self.uniform_buffer.clone(),
        );

        let albedo = self.albedo_texture
            .map(|handle| textures.get(handle).clone())
            .unwrap_or(self.default_textures.white.clone());

        let normal = self.normal_texture
            .map(|handle| textures.get(handle).clone())
            .unwrap_or(self.default_textures.flat_normal.clone());

        let roughness = self.roughness_texture
            .map(|handle| textures.get(handle).clone())
            .unwrap_or(self.default_textures.gray.clone());

        let metallic = self.metallic_texture
            .map(|handle| textures.get(handle).clone())
            .unwrap_or(self.default_textures.black.clone());

        descriptor_manager.add_image_view_sampler(
            &mut writes,
            1,
            albedo.view.clone(),
            albedo.sampler.clone(),
        );

        descriptor_manager.add_image_view_sampler(
            &mut writes,
            2,
            normal.view.clone(),
            normal.sampler.clone(),
        );

        descriptor_manager.add_image_view_sampler(
            &mut writes,
            3,
            roughness.view.clone(),
            roughness.sampler.clone(),
        );

        descriptor_manager.add_image_view_sampler(
            &mut writes,
            4,
            metallic.view.clone(),
            metallic.sampler.clone(),
        );

        self.descriptor_set = Some(
            descriptor_manager.create_set(
                pipeline,
                1,
                writes,
            )
        );
    }

    pub fn recreate_pipeline(
        &mut self,
        device: Arc<Device>,
        render_pass: Arc<RenderPass>, 
        viewport: Viewport,
        samples: SampleCount,
        enable_depth: bool,
        shaders: &ShaderStorage,
        textures: &AssetStorage<Arc<Texture>>,
        descriptor_manager: &DescriptorManager,
    ) {
        let vs = shaders.get(self.vertex_shader.clone());
        let fs = shaders.get(self.fragment_shader.clone());

        let pipeline = PipelineConstructor::get_pipeline::<OsmiumVertex>(
            device, 
            vs, fs, 
            render_pass, 
            viewport, 
            samples, 
            enable_depth
        );

        self.pipeline = Some(pipeline);

        self.recreate_descriptor_set(
            textures,
            descriptor_manager,
        );
    }

    pub fn get_pipeline(&self) -> Arc<GraphicsPipeline> {
        match &self.pipeline {
            Some(p) => p.clone(),
            None => panic!("Pipeline not yet created!")
        }
    }

    pub fn get_descriptor_set(&self) -> Arc<PersistentDescriptorSet> {
        match &self.descriptor_set {
            Some(ds) => ds.clone(),
            None => panic!("Descriptor Set not yet created!")
        }
    }
}