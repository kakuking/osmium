use std::sync::Arc;

use vulkano::{
    buffer::BufferContents, 
    device::Device, 
    image::SampleCount, 
    pipeline::{
        GraphicsPipeline, 
        PipelineLayout, 
        PipelineShaderStageCreateInfo, 
        graphics::{
            GraphicsPipelineCreateInfo, 
            color_blend::{
                ColorBlendAttachmentState, 
                ColorBlendState
            }, 
            depth_stencil::{
                DepthState, 
                DepthStencilState
            }, 
            input_assembly::InputAssemblyState, multisample::MultisampleState, 
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
    }, 
    render_pass::{
        RenderPass, 
        Subpass
    }, 
    shader::ShaderModule
};

pub struct PipelineConstructor {

}

impl PipelineConstructor {
    pub fn get_pipeline<V>(
        device: Arc<Device>,
        vs: Arc<ShaderModule>, 
        fs: Arc<ShaderModule>, 
        render_pass: Arc<RenderPass>, 
        viewport: Viewport,
        samples: SampleCount,
        enable_depth: bool
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

        let depth_stencil_state = if enable_depth {
            Some(
                DepthStencilState {
                    depth: Some(DepthState::simple()),
                    ..Default::default()
                }
            )
        } else {
            None
        };

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
                depth_stencil_state: depth_stencil_state,
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        ).unwrap()
    }
}