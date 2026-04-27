use std::sync::Arc;

use vulkano::{Validated, VulkanError, VulkanLibrary, buffer::{BufferContents, BufferUsage, Subbuffer}, command_buffer::{AutoCommandBufferBuilder, CommandBufferExecFuture, CommandBufferUsage, PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo, allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo}}, device::{Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags, physical::{PhysicalDevice, PhysicalDeviceType}}, image::{Image, ImageUsage}, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, memory::allocator::{MemoryTypeFilter, StandardMemoryAllocator}, pipeline::{GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo, graphics::{GraphicsPipelineCreateInfo, color_blend::{ColorBlendAttachmentState, ColorBlendState}, input_assembly::InputAssemblyState, multisample::MultisampleState, rasterization::RasterizationState, vertex_input::{Vertex, VertexDefinition}, viewport::{Viewport, ViewportState}}, layout::PipelineDescriptorSetLayoutCreateInfo}, render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass}, shader::ShaderModule, swapchain::{self, PresentFuture, Surface, Swapchain, SwapchainAcquireFuture, SwapchainCreateInfo, SwapchainPresentInfo}, sync::{self, GpuFuture, future::{FenceSignalFuture, JoinFuture}}};
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}};

use crate::core::{buffer_manager::BufferManager, descriptor_manager::DescriptorManager, image_manager::ImageManager, shader_manager::ShaderManager, window_manager::WindowManager};

pub struct Application {
    pub window_manager: WindowManager,
    pub instance: Arc<Instance>,
    pub physical_device: Arc<PhysicalDevice>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub memory_allocator: Arc<StandardMemoryAllocator>,
    pub buffer_manager: BufferManager,
    pub image_manager: ImageManager,
    pub command_buffer_allocator: StandardCommandBufferAllocator,
    pub shader_manager: Arc<ShaderManager>,
    pub descriptor_manager: DescriptorManager,
    pub swapchain: Arc<Swapchain>,
    pub swapchain_images: Vec<Arc<Image>>,
}

type FenceType = FenceSignalFuture<PresentFuture<CommandBufferExecFuture<JoinFuture<Box<dyn GpuFuture>, SwapchainAcquireFuture>>>>;

pub struct RenderData {
    vertex_buffer: Subbuffer<[MyVertex]>,
    render_pass: Arc<RenderPass>,
    framebuffers: Vec<Arc<Framebuffer>>,
    viewport: Viewport,
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
    pipeline: Arc<GraphicsPipeline>,
    command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,
    window_resized: bool,
    recreate_swapchain: bool,
    previous_fence_i: usize,
    fences: Vec<Option<Arc<FenceType>>>
}

impl Application {
    pub fn init(event_loop: &EventLoop<()>, enable_validation: bool) -> Self {
        let library = VulkanLibrary::new()
            .expect("No local vulkan lib or DLL");

        let mut enabled_layers: Vec<String> = Vec::new();

        let mut window_manager = WindowManager::init(
            event_loop
        );
        let enabled_extensions = window_manager.get_required_extensions();

        if enable_validation {
            enabled_layers.push("VK_LAYER_KHRONOS_validation".to_string());
        }

        let instance: Arc<Instance> = Instance::new(
            library, 
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions,
                enabled_layers,
                ..Default::default()
            },
        ).expect("Failed to create an instance");

        window_manager.create_surface(instance.clone());

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let (physical_device, queue_family_index) = Self::select_physical_device(
            instance.clone(), 
            window_manager.get_surface(), 
            &device_extensions
        );


        let (device, mut queues) = Device::new(
            physical_device.clone(), 
            DeviceCreateInfo {
                queue_create_infos: vec![
                    QueueCreateInfo {
                        queue_family_index,
                        ..Default::default()
                    }
                ],
                enabled_extensions: device_extensions,
                ..Default::default()
            }
        )
        .expect("Failed to create a device");
    
        let queue = queues.next().unwrap();

        let memory_allocator = Arc::new(
            StandardMemoryAllocator::new_default(
                device.clone()
            )
        );

        let buffer_manager = BufferManager::init(memory_allocator.clone());

        let image_manager = ImageManager::init(memory_allocator.clone());

        let command_buffer_allocator = 
                StandardCommandBufferAllocator::new(
                device.clone(), 
                StandardCommandBufferAllocatorCreateInfo::default()
            );

        let shader_manager = Arc::new(
                ShaderManager::new(
                device.clone()
            )
        );

        let descriptor_manager: DescriptorManager = DescriptorManager::new(device.clone());

        let caps = physical_device
            .surface_capabilities(&window_manager.get_surface(), Default::default())
            .expect("Failed to get surface capabilities");

        let dims = window_manager.get_window().inner_size();
        let composite_alpha = caps
            .supported_composite_alpha.into_iter().next().unwrap();
        let image_format = physical_device
            .surface_formats(&window_manager.get_surface(), Default::default())
            .unwrap()[0]
            .0;

        let (swapchain, swapchain_images) = Swapchain::new(
            device.clone(),
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

        Self {
            instance,
            window_manager,
            physical_device,
            device,
            queue,
            memory_allocator,
            buffer_manager,
            image_manager,
            command_buffer_allocator,
            shader_manager,
            descriptor_manager,
            swapchain,
            swapchain_images
        }
    }

    fn select_physical_device(
        instance: Arc<Instance>, 
        surface: Arc<Surface>, 
        device_extensions: &DeviceExtensions
    ) -> (Arc<PhysicalDevice>, u32) {
        instance
            .enumerate_physical_devices()
            .expect("Couldnt enumerate devices")
            .filter(|p| {
                p.supported_extensions()
                    .contains(&device_extensions)
            })
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.contains(
                            QueueFlags::GRAPHICS
                        ) && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|q| (p, q as u32))
            })
            .min_by_key(|(p, _)| {
                match p.properties().device_type {
                    PhysicalDeviceType::DiscreteGpu => 0,
                    PhysicalDeviceType::IntegratedGpu => 1,
                    PhysicalDeviceType::VirtualGpu => 2,
                    PhysicalDeviceType::Cpu =>3,
                    _ => 4,
                }
            })
            .expect("No available devices")
    }

    fn get_render_pass(&self) -> Arc<RenderPass> {
        vulkano::single_pass_renderpass!(
            self.device.clone(),
            attachments: {
                color: {
                    format: self.swapchain.image_format(),
                    samples: 1,
                    load_op: Clear,
                    store_op: Store
                },
            },
            pass: {
                color: [color],
                depth_stencil: {}
            },
        )
        .unwrap()
    }

    fn get_framebuffers(&self, render_pass: Arc<RenderPass>) -> Vec<Arc<Framebuffer>> {
        self.swapchain_images
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

    fn get_pipeline(&self, vs: Arc<ShaderModule>, fs: Arc<ShaderModule>, render_pass: Arc<RenderPass>, viewport: Viewport) -> Arc<GraphicsPipeline> {
        let vs = vs.entry_point("main").unwrap();
        let fs = fs.entry_point("main").unwrap();

        let vertex_input_state = MyVertex::per_vertex()
            .definition(&vs.info().input_interface)
            .unwrap();

        let stages = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];

        let layout = PipelineLayout::new(
            self.device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
            .into_pipeline_layout_create_info(self.device.clone())
            .unwrap(),
        )
        .unwrap();

        let subpass = Subpass::from(
            render_pass.clone(), 0
        ).unwrap();

        GraphicsPipeline::new(
            self.device.clone(),
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
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(), 
                    ColorBlendAttachmentState::default())
                ),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        ).unwrap()
    }

    fn get_command_buffers<T: BufferContents>(
        &self, 
        pipeline: Arc<GraphicsPipeline>,
        framebuffers: &Vec<Arc<Framebuffer>>,
        vertex_buffer: &Subbuffer<[T]>
    ) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
        framebuffers
            .iter()
            .map(|framebuffer| {
                let mut builder = AutoCommandBufferBuilder::primary(
                    &self.command_buffer_allocator, self.queue.queue_family_index(), CommandBufferUsage::MultipleSubmit)
                    .unwrap();
                
                builder.begin_render_pass(
                    RenderPassBeginInfo {
                        clear_values: vec![Some([0.1, 0.1, 0.1, 1.0].into())],
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

    pub unsafe fn begin(&mut self) -> RenderData {
        let triangles = vec![
            MyVertex { position: [-0.5, -0.5] },
            MyVertex { position: [ 0.0,  0.5] },
            MyVertex { position: [ 0.5, -0.5] },
        ];

        let vertex_buffer: Subbuffer<[MyVertex]> = self.buffer_manager.create_buffer_from_iter(
            triangles,
            Some(BufferUsage::VERTEX_BUFFER),
            Some(MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE)
        );

        let render_pass: Arc<RenderPass> = self.get_render_pass();
    
        let framebuffers: Vec<Arc<Framebuffer>> = self.get_framebuffers(render_pass.clone());

        let viewport: Viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [1024.0, 1024.0],
            depth_range: 0.0..=1.0
        };

        let vs: Arc<ShaderModule> = unsafe {
            self.shader_manager.create_vertex_shader("./shaders/vertex.glsl")
        };
        let fs: Arc<ShaderModule> = unsafe {
            self.shader_manager.create_fragment_shader("./shaders/fragment.glsl")
        };
        
        let pipeline: Arc<GraphicsPipeline> = self.get_pipeline(
            vs.clone(),
            fs.clone(),
            render_pass.clone(),
            viewport.clone(),
        );

        let command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>> = self.get_command_buffers(
            pipeline.clone(),
            &framebuffers,
            &vertex_buffer,
        );

        let window_resized = false;
        let recreate_swapchain = false;

        let frames_in_flight: usize = self.swapchain_images.len();
        let fences: Vec<Option<Arc<FenceType>>> = vec![None; frames_in_flight];
        let previous_fence_i: usize = 0;

        RenderData {
            vertex_buffer,
            render_pass,
            framebuffers,
            viewport,
            vs,
            fs,
            pipeline,
            command_buffers,
            window_resized,
            recreate_swapchain,
            fences,
            previous_fence_i,
        }
    }

    pub unsafe fn handle_event(&mut self, event: Event<()>, control_flow: &mut ControlFlow, render_data: &mut RenderData) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            },
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                render_data.window_resized = true;
            },
            Event::LoopDestroyed => {
                println!("No errors occured!");
            },
            Event::MainEventsCleared => {
                if render_data.window_resized || render_data.recreate_swapchain {
                    render_data.recreate_swapchain = false;

                    let new_dims = self.window_manager.get_window().inner_size();

                    let (new_swapchain, new_images) = self.swapchain.recreate(SwapchainCreateInfo {
                        image_extent: new_dims.into(),
                        ..self.swapchain.create_info()
                    }).expect("Failed to recreate swapchain: {e}");

                    self.swapchain = new_swapchain;
                    self.swapchain_images = new_images;

                    render_data.framebuffers = self.get_framebuffers(self.get_render_pass());

                    if render_data.window_resized {
                        render_data.window_resized = false;

                        render_data.viewport.extent = new_dims.into();

                        render_data.pipeline = self.get_pipeline(
                            render_data.vs.clone(), 
                            render_data.fs.clone(), 
                            render_data.render_pass.clone(), 
                            render_data.viewport.clone()
                        );

                        render_data.command_buffers = self.get_command_buffers(
                            render_data.pipeline.clone(), 
                            &render_data.framebuffers, 
                            &render_data.vertex_buffer
                        );
                    }
                }
            
                let (image_i, suboptimal, acquire_future) = match swapchain::acquire_next_image(self.swapchain.clone(), None).map_err(Validated::unwrap) {
                    Ok(r) => r,
                    Err(VulkanError::OutOfDate) => {
                        render_data.recreate_swapchain = true;
                        return;
                    },
                    Err(e) => panic!("Failed to acquire next image {e}"),
                };

                if suboptimal {
                    render_data.recreate_swapchain = true;
                }

                if let Some(image_fence) = &render_data.fences[image_i as usize] {
                    image_fence.wait(None).unwrap();
                }

                let previous_future = match render_data.fences[render_data.previous_fence_i].clone() {
                    None => {
                        let mut now = sync::now(self.device.clone());
                        now.cleanup_finished();

                        now.boxed()
                    }
                    Some(fence) => fence.boxed()
                };

                let future = previous_future
                    .join(acquire_future)
                    .then_execute(
                        self.queue.clone(), 
                        render_data.command_buffers[image_i as usize].clone()
                    )
                    .unwrap()
                    .then_swapchain_present(
                        self.queue.clone(), 
                        SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_i)
                    )
                    .then_signal_fence_and_flush();

                render_data.fences[image_i as usize] = match future.map_err(
                    Validated::unwrap
                ) {
                    Ok(value) => Some(Arc::new(value)),
                    Err(VulkanError::OutOfDate) => {
                        render_data.recreate_swapchain = true;
                        None
                    },
                    Err(e) => {
                        println!("Failed to flush future: {e}");
                        None
                    }
                };

                render_data.previous_fence_i = image_i as usize;
            }
            _ => ()
        }
    }
}

#[derive(BufferContents, Vertex)]
#[repr(C)]
struct MyVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2]
}