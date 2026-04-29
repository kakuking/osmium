use std::sync::Arc;

use vulkano::{
    format::Format, 
    image::{
        Image, 
        ImageUsage
    }, 
    pipeline::graphics::viewport::Viewport, 
    render_pass::{
        Framebuffer, 
        FramebufferCreateInfo, 
        RenderPass
    }, swapchain::{
        Swapchain, 
        SwapchainCreateInfo
    }
};

use crate::engine::{
    renderer::{
        image_manager::ImageManager, 
        vulkan_context::VulkanContext
    }, 
    window::window_manager::WindowManager
};

pub struct SwapchainManager {
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<Image>>,
    framebuffers: Vec<Arc<Framebuffer>>,
    viewport: Viewport
}

impl SwapchainManager {
    pub fn new(
        vulkan_context: &VulkanContext,
        window_manager: &WindowManager,
        image_format: Format,
        render_pass: Arc<RenderPass>,
    ) -> Self {
        let caps = vulkan_context
            .physical_device
            .surface_capabilities(&window_manager.get_surface(), Default::default())
            .expect("Failed to get surface capabilities");

        let dims = window_manager
            .get_window()
            .inner_size();

        let composite_alpha = caps
            .supported_composite_alpha.into_iter().next().unwrap();

        let (swapchain, images) = Swapchain::new(
            vulkan_context.get_device(),
            window_manager.get_surface(),
            SwapchainCreateInfo {
                min_image_count: caps.min_image_count + 1,
                image_format,
                image_extent: dims.into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha,
                ..Default::default()
            },
        )
        .unwrap();

        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [dims.width as f32, dims.height as f32],
            depth_range: 0.0..=1.0,
        };

        let framebuffers = Self::create_framebuffers(
            &images, 
            render_pass
        );

        Self {
            swapchain,
            images,
            framebuffers,
            viewport,
        }
    }

    fn create_framebuffers(
        swapchain_images: &Vec<Arc<Image>>,
        render_pass: Arc<RenderPass>
    ) -> Vec<Arc<Framebuffer>> {
        swapchain_images
            .iter()
            .map(|image| {
                let view = ImageManager::get_image_view(image.clone());

                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>()
    }

    pub fn recreate(
        &mut self, 
        window_manager: &WindowManager, 
        render_pass: Arc<RenderPass>
    ) {
        let dims = window_manager
            .get_window()
            .inner_size();

        let (swapchain, images) = self.swapchain
            .recreate(SwapchainCreateInfo {
                image_extent: dims.into(),
                ..self.swapchain.create_info()
            })
            .expect("Failed to recreate swapchain");

        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [dims.width as f32, dims.height as f32],
            depth_range: 0.0..=1.0,
        };

        let framebuffers = Self::create_framebuffers(
            &images, 
            render_pass
        );

        self.swapchain = swapchain;
        self.images = images;
        self.viewport = viewport;
        self.framebuffers = framebuffers;
    }

    pub fn get_swapchain(&self) -> Arc<Swapchain> {
        self.swapchain.clone()
    }

    pub fn get_swapchain_images(&self) -> &Vec<Arc<Image>> {
        &self.images
    }

    pub fn get_framebuffers(&self) -> &Vec<Arc<Framebuffer>> {
        &self.framebuffers
    }

    pub fn get_viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn get_image_format(&self) -> Format {
        self.swapchain.image_format()
    }
}