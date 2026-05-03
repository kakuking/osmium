use std::sync::Arc;

use vulkano::{
    Validated, VulkanError, buffer::IndexBuffer, command_buffer::{
        AutoCommandBufferBuilder, 
        CommandBufferExecFuture, 
        CommandBufferUsage, 
        PrimaryAutoCommandBuffer, 
        RenderPassBeginInfo, 
        SubpassBeginInfo, 
        SubpassContents, 
        SubpassEndInfo
    }, format::{
        ClearValue, Format
    }, pipeline::{
        GraphicsPipeline, Pipeline, PipelineBindPoint
    }, render_pass::{
        Framebuffer, RenderPass
    }, swapchain::{
        self, 
        PresentFuture, 
        SwapchainAcquireFuture, 
        SwapchainPresentInfo
    }, sync::{
        self, 
        GpuFuture, 
        future::{
            FenceSignalFuture, 
            JoinFuture
        }
    }
};

use crate::engine::{
    config::config::RendererConfig, ecs::components::renderable::ObjectPushConstants, renderer::{
        buffer_manager::BufferManager, 
        descriptor_manager::DescriptorManager, 
        image_manager::ImageManager, 
        render_pass_constructor::RenderPassConstructor, 
        shader_manager::ShaderManager, 
        swapchain_manager::SwapchainManager, 
        vulkan_context::VulkanContext
    }, scene::{
        asset_manager::AssetManager, render_item::RenderItem
    }, window::window_manager::WindowManager
};

type FenceType = FenceSignalFuture<
    PresentFuture<
        CommandBufferExecFuture<
            JoinFuture<Box<dyn GpuFuture>, SwapchainAcquireFuture>
        >
    >
>;

struct FrameState {
    recreate_swapchain: bool,
    previous_fence_i: usize,
    fences: Vec<Option<Arc<FenceType>>>
}

impl FrameState {
    fn new(frames_in_flight: usize) -> Self {
        Self {
            recreate_swapchain: false,
            previous_fence_i: 0,
            fences: vec![None; frames_in_flight],
        }
    }

    fn request_swapchain_recreate(&mut self) {
        self.recreate_swapchain = true;
    }

    fn clear_swapchain_recreate(&mut self) {
        self.recreate_swapchain = false;
    }
}

pub struct Renderer {
    vulkan_context: VulkanContext,
    render_pass: Arc<RenderPass>,
    
    pub shader_manager: Arc<ShaderManager>,
    pub descriptor_manager: Arc<DescriptorManager>,
    pub buffer_manager: BufferManager,
    pub image_manager: ImageManager,
    
    frame_state: FrameState,
    swapchain_manager: SwapchainManager,
    command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,

    // scene: Scene
}

impl Renderer {
    pub fn init(
        window_manager: &mut WindowManager,
        config: &RendererConfig,
        // scene: Scene,
        render_items: &Vec<RenderItem>,
        assets: &mut AssetManager,
    ) -> Renderer
    {

        let vulkan_context = VulkanContext::create(
            window_manager, 
            &config
        );

        let buffer_manager = BufferManager::init(
            vulkan_context.get_memory_allocator()
        );

        let image_manager = ImageManager::init(
            vulkan_context.get_memory_allocator(),
        );

        let shader_manager = Arc::new(
            ShaderManager::new(
                vulkan_context.get_device()
            )
        );

        let descriptor_manager: Arc<DescriptorManager> = Arc::new(
            DescriptorManager::new(
                vulkan_context.get_device()
            )
        );
        
        let image_format = vulkan_context
            .physical_device
            .surface_formats(&window_manager.get_surface(), Default::default())
            .unwrap()[0]
            .0;

        let depth_format = Format::D32_SFLOAT;

        let render_pass = RenderPassConstructor::create_render_pass(
            &config, 
            vulkan_context.get_device(), 
            image_format, 
            depth_format
        );

        let swapchain_manager = SwapchainManager::new(
            &config,
            &vulkan_context,
            window_manager,
            image_format,
            depth_format,
            render_pass.clone(),
            config.render_pass.samples
        );

        let frame_state = FrameState::new(
            swapchain_manager.get_swapchain_images().len()
        );

        for mesh in assets.meshes.iter_mut() {
            mesh.create_buffers(&buffer_manager);
        }

        assets.create_materials(
            &shader_manager, 
            &image_manager, 
            &buffer_manager, 
            &vulkan_context.command_buffer_allocator, 
            vulkan_context.queue.clone(), 
            vulkan_context.memory_allocator.clone()
        );

        let shaders = &assets.shaders;
        let textures = &assets.textures;
        let materials = &mut assets.materials;

        for material in materials.iter_mut() {
            material.recreate_pipeline(
                vulkan_context.get_device(),
                render_pass.clone(),
                swapchain_manager.get_viewport().clone(),
                swapchain_manager.get_samples(),
                config.enable_depth,
                shaders,
                textures,
                &descriptor_manager
            );
        }

        let command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>> = Self::create_command_buffers(
            &vulkan_context, 
            swapchain_manager.enable_depth(),
            config.render_pass.samples,
            swapchain_manager.get_framebuffers(), 
            render_items,
            assets
        );

        Self {
            vulkan_context,
            render_pass,

            shader_manager,
            descriptor_manager,
            buffer_manager,
            image_manager,
            command_buffers,

            swapchain_manager,
            frame_state,
            // scene,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.frame_state.request_swapchain_recreate();
    }

    pub fn rebuild_command_buffers(
        &mut self,
        render_items: &Vec<RenderItem>,
        assets: &AssetManager,
    ) {
        self.command_buffers = Self::create_command_buffers(
            &self.vulkan_context,
            self.swapchain_manager.enable_depth(),
            self.swapchain_manager.get_samples() as u32,
            self.swapchain_manager.get_framebuffers(),
            render_items,
            assets,
        );
    }

    pub fn recreate_swapchain(
        &mut self, 
        window_manager: &WindowManager,
        assets: &mut AssetManager,
        render_items: &Vec<RenderItem>,
    ) {
        let recreated = self.swapchain_manager.recreate(
            &self.vulkan_context,
            window_manager,
            self.render_pass.clone()
        );

        if !recreated {
            return;
        }

        self.frame_state.clear_swapchain_recreate();

        let shaders = &assets.shaders;
        let textures = &assets.textures;

        for material in assets.materials.iter_mut() {
            material.recreate_pipeline(
                self.vulkan_context.get_device(), 
                self.render_pass.clone(), 
                self.swapchain_manager.get_viewport().clone(), 
                self.swapchain_manager.get_samples(), 
                self.swapchain_manager.enable_depth(),
                shaders,
                textures,
                &self.descriptor_manager
            );
        }

        // let render_items = self.scene.get_render_items();

        self.command_buffers = Self::create_command_buffers(
            &self.vulkan_context,
            self.swapchain_manager.enable_depth(),
            self.swapchain_manager.get_samples() as u32,
            self.swapchain_manager.get_framebuffers(),
            &render_items,
            assets
        );

        self.frame_state.fences = vec![None; self.swapchain_manager.get_swapchain_images().len()];
        self.frame_state.previous_fence_i = 0;
    }

    pub fn render(
        &mut self, 
        window_manager: &WindowManager,
        assets: &mut AssetManager,
        render_items: &Vec<RenderItem>,
    ) {
        if self.frame_state.recreate_swapchain {
            self.recreate_swapchain(
                window_manager,
                assets,
                render_items
            );
        }

        let swapchain = self.swapchain_manager.get_swapchain();

        let (image_i, suboptimal, acquire_future) = match swapchain::acquire_next_image(
            swapchain.clone(), 
            None
        ).map_err(Validated::unwrap) {
            Ok(r) => r,
            Err(VulkanError::OutOfDate) => {
                self.frame_state.request_swapchain_recreate();
                return;
            },
            Err(e) => panic!("Failed to acquire next image {e}"),
        };

        if suboptimal {
            self.frame_state.request_swapchain_recreate();
        }

        if let Some(image_fence) = &self.frame_state.fences[image_i as usize] {
            image_fence.wait(None).unwrap();
        }

        let previous_future = match self.frame_state.fences[self.frame_state.previous_fence_i].clone() {
            None => {
                let mut now = sync::now(
                    self.vulkan_context.get_device()
                );
                now.cleanup_finished();
                now.boxed()
            }
            Some(fence) => fence.boxed()
        };

        let queue = self.vulkan_context.get_queue();

        let future = previous_future
            .join(acquire_future)
            .then_execute(
                queue.clone(), 
                self.command_buffers[image_i as usize].clone()
            )
            .unwrap()
            .then_swapchain_present(
                queue.clone(), 
                SwapchainPresentInfo::swapchain_image_index(swapchain.clone(), image_i)
            )
            .then_signal_fence_and_flush();

        self.frame_state.fences[image_i as usize] = match future.map_err(
            Validated::unwrap
        ) {
            Ok(value) => Some(Arc::new(value)),
            Err(VulkanError::OutOfDate) => {
                self.frame_state.request_swapchain_recreate();
                None
            },
            Err(e) => {
                println!("Failed to flush future: {e}");
                None
            }
        };

        self.frame_state.previous_fence_i = image_i as usize;
    } 

    fn create_command_buffers(
        vulkan_context: &VulkanContext,
        enable_depth: bool,
        msaa: u32,
        framebuffers: &Vec<Arc<Framebuffer>>,
        render_items: &Vec<RenderItem>,
        assets: &AssetManager
    ) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
        framebuffers
            .iter()
            .map(|framebuffer| {
                Self::create_one_command_buffer(
                    vulkan_context, 
                    enable_depth,
                    msaa,
                    framebuffer.clone(), 
                    render_items,
                    assets
                )
            })
            .collect()
    }

    fn create_one_command_buffer(
        vulkan_context: &VulkanContext,
        enable_depth: bool,
        msaa: u32,
        framebuffer: Arc<Framebuffer>,
        render_items: &Vec<RenderItem>,
        assets: &AssetManager
    ) -> Arc<PrimaryAutoCommandBuffer> {
        let clear_color = [0.5, 0.5, 0.5, 1.0];

        let clear_values: Vec<Option<ClearValue>> = match (enable_depth, msaa) {
            (false, 1) => {
                vec![Some(clear_color.into())]
            },
            (true, 1) => {
                vec![
                    Some(clear_color.into()),
                    Some(1.0f32.into()),
                ]
            },
            (false, _) => {
                vec![
                    Some(clear_color.into()),
                    None,
                ]
            },
            (true, _) => {
                vec![
                    Some(clear_color.into()),
                    None,
                    Some(1.0f32.into())
                ]
            }
        };

        let mut builder = AutoCommandBufferBuilder::primary(
            &vulkan_context.command_buffer_allocator, 
            vulkan_context.queue.queue_family_index(), 
            CommandBufferUsage::MultipleSubmit
        )
        .unwrap();
        
        builder.begin_render_pass(
            RenderPassBeginInfo {
                clear_values: clear_values.clone(),
                ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
            },
            SubpassBeginInfo {
                contents: SubpassContents::Inline,
                ..Default::default()
            },
            )
            .unwrap();

        for item in render_items {
            let mesh = item.get_mesh(assets);
            let material = item.get_material(assets);

            let pipeline: Arc<GraphicsPipeline> = material.get_pipeline();

            builder
                .bind_pipeline_graphics(pipeline.clone())
                .unwrap()
                .bind_descriptor_sets(
                    PipelineBindPoint::Graphics, 
                    pipeline.layout().clone(), 
                    1, 
                    material.get_descriptor_set()
                )
                .unwrap()
                .push_constants(
                    pipeline.layout().clone(), 
                    0, 
                ObjectPushConstants {
                    model: item.model_matrix
                })
                .unwrap()
                .bind_vertex_buffers(0, mesh.get_vertex_buffer())
                .unwrap();

            if let Some(idx_buffer) = mesh.index_buffer.clone() {
                builder
                    .bind_index_buffer(
                        IndexBuffer::from(idx_buffer)
                    )
                    .unwrap()
                    .draw_indexed(mesh.get_num_indices(), 1, 0, 0, 0)
                    .unwrap();
            } else {
                builder
                    .draw(mesh.get_num_vertices(), 1, 0, 0)
                    .unwrap();
            }
        }

        builder
            .end_render_pass(SubpassEndInfo::default())
            .unwrap();

        builder
            .build()
            .unwrap()
    }
}

