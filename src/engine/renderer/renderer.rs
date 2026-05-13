use std::sync::Arc;

use vulkano::{
    Validated, VulkanError, buffer::IndexBuffer, command_buffer::{
        AutoCommandBufferBuilder, 
        CommandBufferUsage, 
        PrimaryAutoCommandBuffer, 
        RenderPassBeginInfo, 
        SubpassBeginInfo, 
        SubpassContents, 
        SubpassEndInfo
    }, descriptor_set::PersistentDescriptorSet, format::{
        ClearValue, Format
    }, pipeline::{
        GraphicsPipeline, Pipeline, PipelineBindPoint
    }, render_pass::{
        Framebuffer, RenderPass
    }, swapchain::{
        self, 
        PresentFuture, 
        SwapchainPresentInfo
    }, sync::{
        self, 
        GpuFuture, 
        future::{
            FenceSignalFuture, 
        }
    }
};

use crate::{application::gui::OsmiumGUI, engine::{
    config::renderer_config::RendererConfig, 
    ecs::components::renderable::{
        ColorPushConstants, 
        ShadowPushConstants
    }, 
    renderer::{
        buffer_manager::BufferManager, 
        descriptor_manager::DescriptorManager, 
        global_resources::{
            GlobalResources, 
            RenderGlobals
        }, 
        image_manager::ImageManager, 
        render_pass_constructor::RenderPassConstructor, 
        shader_manager::ShaderManager, 
        shadow_manager::ShadowManager, 
        swapchain_manager::SwapchainManager, 
        vulkan_context::VulkanContext
    }, scene::{
        asset_manager::{
            AssetManager, ShaderKindKey
        }, 
        render_item::RenderItem
    }, 
    window::window_manager::WindowManager
}};

pub type OsmiumFuture = FenceSignalFuture<
    PresentFuture<Box<dyn GpuFuture>>
>;

struct FrameState {
    recreate_swapchain: bool,
    previous_fence_i: usize,
    fences: Vec<Option<Arc<OsmiumFuture>>>
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
    pub vulkan_context: VulkanContext,
    pub render_pass: Arc<RenderPass>,
    
    pub shader_manager: Arc<ShaderManager>,
    pub descriptor_manager: Arc<DescriptorManager>,
    pub buffer_manager: BufferManager,
    pub image_manager: ImageManager,
    
    frame_state: FrameState,
    pub swapchain_manager: SwapchainManager,
    shadow_manager: ShadowManager,

    global_resources: GlobalResources // only 1 as all materials have same set 0
}

impl Renderer {
    pub unsafe fn init(
        window_manager: &mut WindowManager,
        config: &RendererConfig,
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
            &vulkan_context.command_buffer_allocator,
            vulkan_context.get_queue()
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

        let shadow_pass = RenderPassConstructor::create_shadow_render_pass(
            vulkan_context.get_device(), 
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

        unsafe {
            assets.create_materials(
                &shader_manager, 
                &image_manager, 
                &buffer_manager, 
                &vulkan_context.command_buffer_allocator, 
                vulkan_context.queue.clone(), 
            )
        };

        let shadow_vertex_shader = unsafe {
            assets.load_shader(
                match &config.shadow_vertex_shader {
                    Some(p) => p.clone(),
                    None => "./shaders/shadow_vertex.glsl".into()
                }, 
                ShaderKindKey::Vertex, 
                &shader_manager
            )
        };

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
                &descriptor_manager,
                &image_manager
            );
        }

        let default_pipeline: Arc<GraphicsPipeline> = materials
            .iter()
            .next()
            .unwrap()
            .get_pipeline();

        for mesh in assets.meshes.iter_mut() {
            mesh.create_gpu_resources(
                &buffer_manager, 
                default_pipeline.clone(), 
                descriptor_manager.clone(), 
                &image_manager,
                &vulkan_context.command_buffer_allocator,
                vulkan_context.get_queue()
            );
        }

        let shadow_manager = ShadowManager::init(
            &vulkan_context, 
            shadow_pass, 
            depth_format, 
            &image_manager,
            shadow_vertex_shader,
            &assets.shaders
        );

        let global_resources =  GlobalResources::new(
            &buffer_manager,
            &descriptor_manager,
            materials.iter().next().unwrap().get_pipeline(),
            swapchain_manager.get_swapchain_images().len(),
            &shadow_manager.views,
            &shadow_manager.samplers
        );

        Self {
            vulkan_context,
            render_pass,

            shader_manager,
            descriptor_manager,
            buffer_manager,
            image_manager,

            swapchain_manager,
            shadow_manager,
            frame_state,
            
            global_resources,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.frame_state.request_swapchain_recreate();
    }

    pub fn recreate_swapchain(
        &mut self, 
        window_manager: &WindowManager,
        assets: &mut AssetManager,
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
                &self.descriptor_manager,
                &self.image_manager
            );
        }

        self.frame_state.fences = vec![None; self.swapchain_manager.get_swapchain_images().len()];
        self.frame_state.previous_fence_i = 0;
    }

    pub fn render(
        &mut self, 
        window_manager: &WindowManager,
        assets: &mut AssetManager,
        render_items: &Vec<RenderItem>,
        globals: RenderGlobals,
        gui: Option<&mut OsmiumGUI>
    ) {
        if self.frame_state.recreate_swapchain {
            self.recreate_swapchain(
                window_manager,
                assets,
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

        self.global_resources.update(
            image_i as usize, 
            &globals
        );

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

        let frame_i = image_i as usize;

        let command_buffers =  Self::create_command_buffers(
            &self.vulkan_context, 
            self.swapchain_manager.enable_depth(), 
            self.swapchain_manager.get_samples() as u32, 
            self.swapchain_manager.get_framebuffers()[frame_i].clone(), 
            render_items,
            assets,
            self.global_resources.descriptor_set(frame_i),
            &self.shadow_manager,
            &globals,
            gui
        );

        let future = previous_future
            .join(acquire_future)
            .then_execute(
                queue.clone(),
                command_buffers,
            )
            .unwrap()
            .boxed()
            .then_swapchain_present(
                queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(
                    swapchain.clone(),
                    image_i,
                ),
            )
            .then_signal_fence_and_flush();

        self.frame_state.fences[frame_i] = match future.map_err(
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

        self.frame_state.previous_fence_i = frame_i;
    } 

    fn create_command_buffers(
        vulkan_context: &VulkanContext,
        enable_depth: bool,
        msaa: u32,
        framebuffer: Arc<Framebuffer>,
        render_items: &Vec<RenderItem>,
        assets: &AssetManager,
        global_descriptor_set: Arc<PersistentDescriptorSet>,
        shadow_manager: &ShadowManager,
        globals: &RenderGlobals,
        gui: Option<&mut OsmiumGUI>
    ) -> Arc<PrimaryAutoCommandBuffer> {
        let clear_color = [1.0, 1.0, 1.0, 1.0];

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

        if globals.lights.len() > 0 {
            for (idx, light) in globals.lights.iter().enumerate() {
                builder.begin_render_pass(
                    RenderPassBeginInfo {
                        clear_values: vec![Some(1.0f32.into())],
                        ..RenderPassBeginInfo::framebuffer(shadow_manager.get_framebuffers(idx))
                    }, 
                    SubpassBeginInfo {
                        contents: SubpassContents::Inline,
                        ..Default::default()
                    }
                ).unwrap();

                for item in render_items {
                    let mesh = item.get_mesh(assets);

                    let pipeline: Arc<GraphicsPipeline> = shadow_manager.get_pipeline();

                    builder
                        .bind_pipeline_graphics(pipeline.clone())
                        .unwrap()
                        .bind_descriptor_sets(
                            PipelineBindPoint::Graphics, 
                            pipeline.layout().clone(), 
                            2, 
                            mesh.get_descriptor_set()
                        )
                        .unwrap()
                        .push_constants(
                            pipeline.layout().clone(), 
                            0, 
                        ShadowPushConstants {
                            model: item.model_matrix,
                            view_proj: light.view_proj
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
            }
        }
        
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
                    0, 
                    (
                        global_descriptor_set.clone(),
                        material.get_descriptor_set(),
                        mesh.get_descriptor_set()
                    )
                )
                .unwrap()
                .push_constants(
                    pipeline.layout().clone(), 
                    0, 
                ColorPushConstants {
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

        if let Some(gui) = gui {
            let gui_cb = gui.render(
                framebuffer.extent()
            );
    
            builder
                .next_subpass(
                    SubpassEndInfo::default(), 
                    SubpassBeginInfo {
                        contents: SubpassContents::SecondaryCommandBuffers,
                        ..Default::default()
                    },
                )
                .unwrap();
    
            builder
                .execute_commands(gui_cb)
                .unwrap();
        }
        
        builder
            .end_render_pass(SubpassEndInfo::default())
            .unwrap();

        builder
            .build()
            .unwrap()
    }
}

