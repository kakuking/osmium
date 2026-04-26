use std::sync::Arc;

use vulkano::{VulkanLibrary, buffer::{BufferContents, BufferUsage}, command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyImageToBufferInfo, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo, allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo}}, device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags, physical::PhysicalDevice}, format::Format, image::ImageUsage, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, memory::allocator::{MemoryTypeFilter, StandardMemoryAllocator}, pipeline::{GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo, graphics::{GraphicsPipelineCreateInfo, color_blend::{ColorBlendAttachmentState, ColorBlendState}, input_assembly::InputAssemblyState, multisample::MultisampleState, rasterization::RasterizationState, vertex_input::{Vertex, VertexDefinition}, viewport::{Viewport, ViewportState}}, layout::PipelineDescriptorSetLayoutCreateInfo}, render_pass::{Framebuffer, FramebufferCreateInfo, Subpass}, sync::{self, GpuFuture}};
use image::{ImageBuffer, Rgba};

use crate::core::{buffer_manager::BufferManager, descriptor_manager::DescriptorManager, image_manager::ImageManager, shader_manager::ShaderManager};

pub struct Application {
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
}

impl Application {
    pub fn init(enable_validation: bool) -> Self {
        let library = VulkanLibrary::new()
            .expect("No local vulkan lib or DLL");

        let mut enabled_layers: Vec<String> = Vec::new();

        if enable_validation {
            enabled_layers.push("VK_LAYER_KHRONOS_validation".to_string());
        }

        let instance = Instance::new(
            library, 
            InstanceCreateInfo {
                enabled_layers,
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                ..Default::default()
            },
        ).expect("Failed to create an instance");

        // TODO - Device selection
        let physical_device = instance
            .enumerate_physical_devices()
            .expect("Couldnt enumerate devices")
            .next()
            .expect("No available devices");

        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .position(
                |queue_family_properties|
                    queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
            )
            .expect("Couldnt find a graphical queue family") as u32;

        let (device, mut queues) = Device::new(
            physical_device.clone(), 
            DeviceCreateInfo {
                queue_create_infos: vec![
                    QueueCreateInfo {
                        queue_family_index,
                        ..Default::default()
                    }
                ],
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

        Self {
            instance,
            physical_device,
            device,
            queue,
            memory_allocator,
            buffer_manager,
            image_manager,
            command_buffer_allocator,
            shader_manager,
            descriptor_manager
        }
    }

    pub unsafe fn run(&mut self) {
        let buf = self.buffer_manager.create_buffer_from_iter(
            (0..1024*1024*4).map(|_| 0u8), 
            Some(BufferUsage::TRANSFER_DST), 
            Some(MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_RANDOM_ACCESS)
        );

        let triangles = vec![
            MyVertex { position: [-0.5, -0.5] },
            MyVertex { position: [ 0.0,  0.5] },
            MyVertex { position: [ 0.5, -0.5] },
        ];

        let vertex_buffer = self.buffer_manager.create_buffer_from_iter(
            triangles,
            Some(BufferUsage::VERTEX_BUFFER),
            Some(MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE)
        );

        let image = self.image_manager.create_image(
            [1024, 1024, 1], 
            None, 
            None, 
            Some(ImageUsage::COLOR_ATTACHMENT | ImageUsage::TRANSFER_SRC), 
            None
        );

        let view = ImageManager::get_image_view(image.clone());

        let render_pass = vulkano::single_pass_renderpass!(
            self.device.clone(),
            attachments: {
                color: {
                    format: Format::R8G8B8A8_UNORM,
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
        .unwrap();

    
        let framebuffer = Framebuffer::new(
            render_pass.clone(),
            FramebufferCreateInfo {
                attachments: vec![view],
                ..Default::default()
            }
        )
        .unwrap();

        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [1024.0, 1024.0],
            depth_range: 0.0..=1.0
        };

        let vs = unsafe {
            self.shader_manager.create_vertex_shader("./shaders/vertex.glsl")
        };
        let fs = unsafe {
            self.shader_manager.create_fragment_shader("./shaders/fragment.glsl")
        };

        let pipeline = {
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
        };

        let mut builder = AutoCommandBufferBuilder::primary(
            &self.command_buffer_allocator, 
            self.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit
        ).unwrap();

        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.0, 0.5, 0.5, 1.0].into())],
                    ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
                },
                SubpassBeginInfo {
                    contents: SubpassContents::Inline,
                    ..Default::default()
                }
            )
            .unwrap()
            .bind_pipeline_graphics(pipeline.clone())
            .unwrap()
            .bind_vertex_buffers(0, vertex_buffer.clone())
            .unwrap()
            .draw(3, 1, 0, 0)
            .unwrap()
            .end_render_pass(SubpassEndInfo::default())
            .unwrap()
            .copy_image_to_buffer(
                CopyImageToBufferInfo::image_buffer(
                    image.clone(), 
                    buf.clone()
                )
            )
            .unwrap();

        let command_buffer = builder.build().unwrap();

        let future = sync::now(self.device.clone())
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        future.wait(None).unwrap();

        let buffer_content = buf.read().unwrap();
        let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();

        image.save("out.png").unwrap();
    }
}

#[derive(BufferContents, Vertex)]
#[repr(C)]
struct MyVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2]
}