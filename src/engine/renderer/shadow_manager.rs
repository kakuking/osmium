use std::sync::Arc;

use vulkano::{
    format::Format, image::{
        Image, 
        ImageUsage, 
        sampler::{
            Filter, 
            Sampler, 
            SamplerAddressMode, 
            SamplerCreateInfo
        }, 
        view::ImageView
    }, memory::allocator::MemoryTypeFilter, pipeline::GraphicsPipeline, render_pass::{
        Framebuffer, 
        FramebufferCreateInfo, 
        RenderPass
    }, shader::ShaderModule
};

use crate::engine::{
    renderer::{
        global_resources::MAX_LIGHTS, 
        image_manager::ImageManager, 
        pipeline_constructor::PipelineConstructor, 
        vulkan_context::VulkanContext
    }, scene::{
        asset_manager::{
            Handle, 
            ShaderStorage
        }, 
        mesh::OsmiumVertex
    }
};

pub struct ShadowManager {
    pub extent: [u32; 3],
    pub images: Vec<Arc<Image>>,
    pub views: Vec<Arc<ImageView>>,
    pub samplers: Vec<Arc<Sampler>>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub pipeline: Arc<GraphicsPipeline>,
    pub shadow_pass: Arc<RenderPass>
}

impl ShadowManager {
    pub fn init(
        vulkan_context: &VulkanContext,
        shadow_pass: Arc<RenderPass>,
        depth_format: Format,
        image_manager: &ImageManager,
        shadow_vertex_shader: Handle<Arc<ShaderModule>>,
        shader_storage: &ShaderStorage,
    ) -> Self {
        let extent = [2048, 2048, 1];

        let mut images: Vec<Arc<Image>> = Vec::new();
        let mut views: Vec<Arc<ImageView>> = Vec::new();
        let mut samplers: Vec<Arc<Sampler>> = Vec::new();
        let mut framebuffers: Vec<Arc<Framebuffer>> = Vec::new();

        for _ in 0..MAX_LIGHTS {
            let image = image_manager.create_image(
                extent, 
                None, 
                Some(depth_format), 
                Some(
                    ImageUsage::DEPTH_STENCIL_ATTACHMENT |
                    ImageUsage::SAMPLED |
                    ImageUsage::TRANSFER_DST
                ), 
                Some(
                    MemoryTypeFilter::PREFER_DEVICE
                )
            );
    
            let view = ImageManager::get_image_view(image.clone());

            let sampler = Sampler::new(
                vulkan_context.get_device(), 
                SamplerCreateInfo {
                    mag_filter: Filter::Nearest,
                    min_filter: Filter::Nearest,
                    address_mode: [
                        SamplerAddressMode::ClampToEdge,
                        SamplerAddressMode::ClampToEdge,
                        SamplerAddressMode::ClampToEdge,
                    ],
                    ..Default::default()
                },
            )
            .unwrap();

            let framebuffer = Framebuffer::new(
                shadow_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view.clone()],
                    ..Default::default()
                },
            ).unwrap();
    
            images.push(image);
            views.push(view);
            samplers.push(sampler);
            framebuffers.push(framebuffer);
        }


        

        let vs = shader_storage.get(shadow_vertex_shader);

        let pipeline = PipelineConstructor::get_shadow_pipeline::<OsmiumVertex>(
            vulkan_context.get_device(),
            vs, 
            shadow_pass.clone(), 
            extent.clone()
        );

        Self {
            extent,
            images,
            views,
            samplers,
            framebuffers,
            pipeline,
            shadow_pass
        }
    }

    pub fn get_framebuffers(&self, idx: usize) -> Arc<Framebuffer> {
        self.framebuffers[idx].clone()
    }

    pub fn get_pipeline(&self) -> Arc<GraphicsPipeline> {
        self.pipeline.clone()
    }
}