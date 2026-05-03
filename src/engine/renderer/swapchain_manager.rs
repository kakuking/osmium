use std::sync::Arc;

use vulkano::{
    format::Format, image::{
        Image, ImageCreateInfo, ImageType, ImageUsage, SampleCount
    }, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter}, pipeline::graphics::viewport::Viewport, render_pass::{
        Framebuffer, 
        FramebufferCreateInfo, 
        RenderPass
    }, swapchain::{
        // PresentMode, 
        Swapchain, 
        SwapchainCreateInfo
    }
};

use crate::engine::{
    renderer::{
        image_manager::ImageManager, vulkan_context::VulkanContext
    },
    config::{
        config::RendererConfig
    }, 
    window::window_manager::WindowManager
};

pub struct SwapchainManager {
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<Image>>,
    depth_images: Vec<Arc<Image>>,
    msaa_images: Vec<Arc<Image>>,
    framebuffers: Vec<Arc<Framebuffer>>,
    viewport: Viewport,
    samples: SampleCount,
}

impl SwapchainManager {
    pub fn new(
        config: &RendererConfig,
        vulkan_context: &VulkanContext,
        window_manager: &WindowManager,
        image_format: Format,
        depth_format: Format,
        render_pass: Arc<RenderPass>,
        samples: u32
    ) -> Self {

        let samples = Self::sample_count(samples);

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
                image_usage: ImageUsage::COLOR_ATTACHMENT | ImageUsage::TRANSFER_DST,
                composite_alpha,
                // present_mode: PresentMode::Fifo,
                ..Default::default()
            },
        )
        .unwrap();

        let msaa_images = Self::create_msaa_images(
            vulkan_context,
            image_format,
            dims.width,
            dims.height,
            images.len(),
            samples,
        );

        let depth_images = if config.enable_depth {
            Self::create_depth_images(
                vulkan_context, 
                config.enable_depth,
                Some(depth_format), 
                dims.width, 
                dims.height, 
                images.len(), 
                samples
            )
        } else {
            vec![]
        };


        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [dims.width as f32, dims.height as f32],
            depth_range: 0.0..=1.0,
        };

        let framebuffers = Self::create_framebuffers(
            &images, 
            &msaa_images,
            &depth_images,
            render_pass,
            config.enable_depth,
            samples,
        );

        Self {
            swapchain,
            images,
            msaa_images,
            depth_images,
            framebuffers,
            viewport,
            samples
        }
    }

    fn sample_count(samples: u32) -> SampleCount {
        match samples {
            1 => SampleCount::Sample1,
            2 => SampleCount::Sample2,
            4 => SampleCount::Sample4,
            8 => SampleCount::Sample8,
            16 => SampleCount::Sample16,
            32 => SampleCount::Sample32,
            64 => SampleCount::Sample64,
            _ => panic!("Unsupported MSAA sample count: {samples}"),
        }
    }

    fn create_depth_images(
        vulkan_context: &VulkanContext,
        enable_depth: bool,
        depth_format: Option<Format>,
        width: u32,
        height: u32,
        count: usize,
        samples: SampleCount,
    ) -> Vec<Arc<Image>> {
        if !enable_depth {
            return Vec::new();
        }

        (0..count)
            .map(|_| {
                Image::new(
                    vulkan_context.get_memory_allocator(),
                    ImageCreateInfo {
                        image_type: ImageType::Dim2d,
                        format: depth_format.unwrap(),
                        extent: [width, height, 1],
                        usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT,
                        samples,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect()
    }

    fn create_msaa_images(
        vulkan_context: &VulkanContext,
        image_format: Format,
        width: u32,
        height: u32,
        count: usize,
        samples: SampleCount,
    ) -> Vec<Arc<Image>> {
        if samples == SampleCount::Sample1 {
            return  Vec::new();
        }

        (0..count)
            .map(|_| {
                Image::new(
                    vulkan_context.get_memory_allocator(),
                    ImageCreateInfo {
                        image_type: ImageType::Dim2d,
                        format: image_format,
                        extent: [width, height, 1],
                        usage: ImageUsage::COLOR_ATTACHMENT,
                        samples,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                        ..Default::default()
                    }
                )
                .unwrap()
            })
            .collect()
    }

    fn create_framebuffers(
        swapchain_images: &Vec<Arc<Image>>,
        msaa_images: &Vec<Arc<Image>>,
        depth_images: &Vec<Arc<Image>>,
        render_pass: Arc<RenderPass>,
        enable_depth: bool,
        samples: SampleCount,
    ) -> Vec<Arc<Framebuffer>> {
        swapchain_images
            .iter()
            .enumerate()
            .map(|(i, image)| {
                let swapchain_view = ImageManager::get_image_view(
                    image.clone()
                );

                let attachments = match (enable_depth, samples == SampleCount::Sample1) {
                    (false, true) => {
                        vec![swapchain_view]
                    }

                    (true, true) => {
                        let depth_view = ImageManager::get_image_view(depth_images[i].clone());

                        vec![
                            swapchain_view,
                            depth_view,
                        ]
                    }

                    (false, false) => {
                        let msaa_view = ImageManager::get_image_view(msaa_images[i].clone());

                        vec![
                            msaa_view,
                            swapchain_view,
                        ]
                    }

                    (true, false) => {
                        let msaa_view = ImageManager::get_image_view(msaa_images[i].clone());
                        let depth_view = ImageManager::get_image_view(depth_images[i].clone());

                        vec![
                            msaa_view,
                            swapchain_view,
                            depth_view,
                        ]
                    }
                };

                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments,
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>()
    }

    pub fn recreate(
        &mut self,
        vulkan_context: &VulkanContext,
        window_manager: &WindowManager, 
        render_pass: Arc<RenderPass>
    ) -> bool {
        let dims = window_manager
            .get_window()
            .inner_size();

        if dims.width == 0 || dims.height == 0 {
            return false;
        }

        let (swapchain, images) = self.swapchain
            .recreate(SwapchainCreateInfo {
                image_extent: dims.into(),
                ..self.swapchain.create_info()
            })
            .expect("Failed to recreate swapchain");

        let msaa_images = Self::create_msaa_images(
            vulkan_context, 
            self.swapchain.image_format(), 
            dims.width, 
            dims.height, 
            images.len(), 
            self.samples
        );

        let enable_depth = self.depth_images.len() != 0;

        let depth_images = 
            Self::create_depth_images(
                vulkan_context, 
                enable_depth,
                self.get_depth_format(), 
                dims.width, 
                dims.height, 
                images.len(), 
                self.samples
            );

        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [dims.width as f32, dims.height as f32],
            depth_range: 0.0..=1.0,
        };

        let framebuffers = Self::create_framebuffers(
            &images, 
            &msaa_images,
            &depth_images,
            render_pass,
            enable_depth,
            self.samples,
        );

        self.swapchain = swapchain;
        self.images = images;
        self.msaa_images = msaa_images;
        self.viewport = viewport;
        self.framebuffers = framebuffers;

        true
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

    pub fn get_samples(&self) -> SampleCount {
        self.samples
    }

    pub fn enable_depth(&self) -> bool {
        self.depth_images.len() > 0
    }

    fn get_depth_format(&self) -> Option<Format> {
        if self.depth_images.len() == 0 {
            return None;
        }

        Some(
            self.depth_images[0].format()
        )
    }
}