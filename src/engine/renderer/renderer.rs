use std::sync::Arc;

use shaderc::ShaderKind;
use vulkano::{
    Validated, VulkanError, buffer::{BufferContents, BufferUsage, Subbuffer}, command_buffer::{
        AutoCommandBufferBuilder, 
        CommandBufferExecFuture, 
        CommandBufferUsage, 
        PrimaryAutoCommandBuffer, 
        RenderPassBeginInfo, 
        SubpassBeginInfo, 
        SubpassContents, 
        SubpassEndInfo
    }, device::Device, format::{ClearValue, Format}, image::SampleCount, memory::allocator::MemoryTypeFilter, pipeline::{
        GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo, 
        graphics::{
            GraphicsPipelineCreateInfo, 
            color_blend::{
                ColorBlendAttachmentState, 
                ColorBlendState
            }, 
            input_assembly::InputAssemblyState, 
            multisample::MultisampleState, 
            rasterization::RasterizationState, 
            vertex_input::{
                Vertex,
                VertexDefinition
            }, 
            viewport::{
                Viewport, 
                ViewportState
            }
        }, 
        layout::PipelineDescriptorSetLayoutCreateInfo
    }, render_pass::{
        Framebuffer, RenderPass, Subpass
    }, shader::ShaderModule, swapchain::{
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
        renderer::{
            buffer_manager::BufferManager, config::RendererConfig, descriptor_manager::DescriptorManager, image_manager::ImageManager, render_pass_constructor::RenderPassConstructor, shader_manager::ShaderManager, swapchain_manager::SwapchainManager, vulkan_context::VulkanContext
        }, 
        window::window_manager::WindowManager
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


#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct MyVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2]
}

pub struct TriangleRenderObject {
    pub vertex_buffer: Subbuffer<[MyVertex]>,
    pub command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,
}

impl TriangleRenderObject {
    pub fn new(
        vulkan_context: &VulkanContext,
        config: &RendererConfig,
        buffer_manager: &BufferManager,
        pipeline: Arc<GraphicsPipeline>,
        framebuffers: &Vec<Arc<Framebuffer>>,
    ) -> Self {
        let triangles = vec![
            MyVertex { position: [-0.5, -0.5] },
            MyVertex { position: [ 0.0,  0.5] },
            MyVertex { position: [ 0.5, -0.5] },
        ];

        let vertex_buffer: Subbuffer<[MyVertex]> = buffer_manager.create_buffer_from_iter(
            triangles,
            Some(BufferUsage::VERTEX_BUFFER),
            Some(
                MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE
            )
        );

        let command_buffers = Self::create_command_buffers(
            vulkan_context, 
            config,
            pipeline, 
            framebuffers, 
            &vertex_buffer
        );

        Self {
            vertex_buffer,
            command_buffers
        }
    }

    pub fn recreate_command_buffers(
        &mut self,
        vulkan_context: &VulkanContext,
        config: &RendererConfig,
        pipeline: Arc<GraphicsPipeline>,
        framebuffers: &Vec<Arc<Framebuffer>>,
    ) {
        self.command_buffers = Self::create_command_buffers(
            vulkan_context,
            config,
            pipeline,
            framebuffers,
            &self.vertex_buffer
        );
    }

    fn create_command_buffers(
        vulkan_context: &VulkanContext,
        config: &RendererConfig,
        pipeline: Arc<GraphicsPipeline>,
        framebuffers: &Vec<Arc<Framebuffer>>,
        vertex_buffer: &Subbuffer<[MyVertex]>,
    ) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
        let depth = config.render_pass.depth_enabled;
        let msaa = config.render_pass.samples;

        let clear_color = [0.1, 0.1, 0.1, 1.0];

        let clear_values: Vec<Option<ClearValue>> = match (depth, msaa) {
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
        // let clear_values = 

        framebuffers
            .iter()
            .map(|framebuffer| {
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
                    .unwrap()
                    .bind_pipeline_graphics(pipeline.clone())
                    .unwrap()
                    .bind_vertex_buffers(0, vertex_buffer.clone())
                    .unwrap()
                    .draw(vertex_buffer.len() as u32, 1, 0, 0)
                    .unwrap()
                    .end_render_pass(SubpassEndInfo::default())
                    .unwrap();

                builder.build().unwrap()
            })
            .collect()
    }
}

pub struct Renderer {
    vulkan_context: VulkanContext,
    render_pass: Arc<RenderPass>,
    pipeline: Arc<GraphicsPipeline>,
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
    
    pub shader_manager: Arc<ShaderManager>,
    pub descriptor_manager: Arc<DescriptorManager>,
    pub buffer_manager: BufferManager,
    pub image_manager: ImageManager,
    
    swapchain_manager: SwapchainManager,
    frame_state: FrameState,
    render_object: TriangleRenderObject,

    config: RendererConfig,
}

impl Renderer {
    pub fn init(
        window_manager: &mut WindowManager,
        config: RendererConfig
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
            vulkan_context.get_memory_allocator()
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
            &vulkan_context,
            window_manager,
            image_format,
            render_pass.clone(),
            config.render_pass.samples
        );

        let vs: Arc<ShaderModule> = unsafe {
            shader_manager.create_shader(
                &config.vs_path,
                ShaderKind::Vertex,
            )
        };
        let fs: Arc<ShaderModule> = unsafe {
            shader_manager.create_shader(
                &config.fs_path,
                ShaderKind::Fragment,
            )
        };

        let pipeline = Self::get_pipeline::<MyVertex>(
            vulkan_context.get_device(), 
            vs.clone(), fs.clone(), 
            render_pass.clone(), 
            swapchain_manager.get_viewport().clone(),
            swapchain_manager.get_samples()
        );

        let frame_state = FrameState::new(
            swapchain_manager.get_swapchain_images().len()
        );

        let render_object = TriangleRenderObject::new(
            &vulkan_context,
            &config,
            &buffer_manager,
            pipeline.clone(),
            swapchain_manager.get_framebuffers()
        );

        Self {
            vulkan_context,
            render_pass,
            pipeline,
            vs, fs,

            shader_manager,
            descriptor_manager,
            buffer_manager,
            image_manager,

            swapchain_manager,
            frame_state,
            render_object,

            config
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.frame_state.request_swapchain_recreate();
    }

    pub fn recreate_swapchain(&mut self, window_manager: &WindowManager) {
        self.frame_state.clear_swapchain_recreate();

        self.swapchain_manager.recreate(
            &self.vulkan_context,
            window_manager,
            self.render_pass.clone()
        );

        self.pipeline = Self::get_pipeline::<MyVertex>(
            self.vulkan_context.get_device(), 
            self.vs.clone(), self.fs.clone(), 
            self.render_pass.clone(), 
            self.swapchain_manager.get_viewport().clone(),
            self.swapchain_manager.get_samples()
        );

        //recreate command buffers
        self.render_object.recreate_command_buffers(
            &self.vulkan_context, 
            &self.config,
            self.pipeline.clone(), 
            self.swapchain_manager.get_framebuffers()
        );

        self.frame_state.fences = vec![None; self.swapchain_manager.get_swapchain_images().len()];
        self.frame_state.previous_fence_i = 0;
    }

    pub fn render(&mut self, window_manager: &WindowManager) {
        if self.frame_state.recreate_swapchain {
            self.recreate_swapchain(window_manager);
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
                self.render_object.command_buffers[image_i as usize].clone()
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

    fn get_pipeline<V>(
        device: Arc<Device>,
        vs: Arc<ShaderModule>, 
        fs: Arc<ShaderModule>, 
        render_pass: Arc<RenderPass>, 
        viewport: Viewport,
        samples: SampleCount
    ) -> Arc<GraphicsPipeline>
    where
        V: BufferContents + Vertex
    {
        let vs = vs.entry_point("main").unwrap();
        let fs = fs.entry_point("main").unwrap();

        let vertex_input_state = V::per_vertex()
            .definition(&vs.info().input_interface)
            .unwrap();

        let stages: [PipelineShaderStageCreateInfo; 2] = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];

        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
            .into_pipeline_layout_create_info(device.clone())
            .unwrap(),
        )
        .unwrap();

        let subpass = Subpass::from(
            render_pass.clone(), 0
        ).unwrap();

        GraphicsPipeline::new(
            device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState {
                    viewports: [viewport].into_iter().collect(),
                    ..Default::default()
                }),
                rasterization_state: Some(RasterizationState::default()),
                multisample_state: Some(MultisampleState {
                    rasterization_samples: samples,
                    ..Default::default()
                }),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(), 
                    ColorBlendAttachmentState::default())
                ),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        ).unwrap()
    }
}

