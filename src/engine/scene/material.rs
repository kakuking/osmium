use std::sync::Arc;

use shaderc::ShaderKind;
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
    shader::ShaderModule
};

use crate::engine::{
    config::material::MaterialConfig, renderer::{
        buffer_manager::BufferManager, 
        descriptor_manager::DescriptorManager, 
        image_manager::{
            DefaultTextures, 
            ImageManager, 
            Texture
        }, 
        pipeline_constructor::PipelineConstructor, 
        shader_manager::ShaderManager
    }, scene::mesh::OsmiumVertex
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

#[derive(Clone)]
pub struct Material {
    vertex_shader: Arc<ShaderModule>,
    fragment_shader: Arc<ShaderModule>,

    pub name: String,
    pub uniform: MaterialUniform,
    uniform_buffer: Subbuffer<MaterialUniform>,

    albedo_texture: Option<Arc<Texture>>,
    normal_texture: Option<Arc<Texture>>,
    roughness_texture: Option<Arc<Texture>>,
    metallic_texture: Option<Arc<Texture>>,

    pipeline: Option<Arc<GraphicsPipeline>>,
    descriptor_set: Option<Arc<PersistentDescriptorSet>>,

    default_textures: DefaultTextures
}

impl Material {
    pub fn init(
        config: &MaterialConfig,
        shader_manager: &ShaderManager,
        image_manager: &ImageManager,
        buffer_manager: &BufferManager,
        command_buffer_allocator: &StandardCommandBufferAllocator,
        queue: Arc<Queue>,
        memory_allocator: Arc<dyn MemoryAllocator>,
    ) -> Self {
        let vertex_shader: Arc<ShaderModule> = unsafe {
            shader_manager.create_shader(
                &config.vertex_shader,
                ShaderKind::Vertex,
            )
        };
        let fragment_shader: Arc<ShaderModule> = unsafe {
            shader_manager.create_shader(
                &config.fragment_shader,
                ShaderKind::Fragment,
            )
        };

        let albedo_texture = config.textures.albedo
            .as_ref()
            .map(|path| 
                image_manager.load_texture(
                    path, 
                    command_buffer_allocator, 
                    queue.clone()
                )
            );

        let normal_texture = config.textures.normal
            .as_ref()
            .map(|path| 
                image_manager.load_texture(
                    path, 
                    command_buffer_allocator, 
                    queue.clone()
                )
            );

        let roughness_texture = config.textures.roughness
            .as_ref()
            .map(|path| 
                image_manager.load_texture(
                    path, 
                    command_buffer_allocator, 
                    queue.clone()
                )
            );

        let metallic_texture = config.textures.metallic
            .as_ref()
            .map(|path| 
                image_manager.load_texture(
                    path, 
                    command_buffer_allocator, 
                    queue.clone()
                )
            );

        let default_textures = DefaultTextures::new(
            memory_allocator.clone(),
            command_buffer_allocator,
            queue
        );

        let uniform = MaterialUniform {
            base_color: config.params.base_color,

            pbr_params: [
                config.params.roughness,
                config.params.metallic,
                0.0,
                0.0,
            ],

            texture_flags: [
                albedo_texture.is_some() as u32,
                normal_texture.is_some() as u32,
                roughness_texture.is_some() as u32,
                metallic_texture.is_some() as u32,
            ],
        };

        let uniform_buffer = buffer_manager.create_buffer_from_data(
            uniform.clone(), 
            Some(BufferUsage::UNIFORM_BUFFER), 
            Some(
                // MemoryTypeFilter::PREFER_DEVICE |
                MemoryTypeFilter::HOST_SEQUENTIAL_WRITE
            )
        );

        Self {
            name: config.name.clone(),

            vertex_shader, 
            fragment_shader,

            uniform: uniform,
            uniform_buffer: uniform_buffer,

            albedo_texture,
            normal_texture,
            roughness_texture,
            metallic_texture,

            pipeline: None,
            descriptor_set: None,

            default_textures
        }
    }

    pub fn recreate_descriptor_set(
        &mut self,
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
            .as_ref()
            .unwrap_or(&self.default_textures.white);

        let normal = self.normal_texture
            .as_ref()
            .unwrap_or(&self.default_textures.flat_normal);

        let roughness = self.roughness_texture
            .as_ref()
            .unwrap_or(&self.default_textures.gray);

        let metallic = self.metallic_texture
            .as_ref()
            .unwrap_or(&self.default_textures.black);

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
        descriptor_manager: &DescriptorManager,
    ) {
        let pipeline = PipelineConstructor::get_pipeline::<OsmiumVertex>(
            device, 
            self.vertex_shader.clone(), self.fragment_shader.clone(), 
            render_pass, 
            viewport, 
            samples, 
            enable_depth
        );

        self.pipeline = Some(pipeline);

        self.recreate_descriptor_set(
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