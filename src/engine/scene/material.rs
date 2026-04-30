use std::sync::Arc;

use shaderc::ShaderKind;
use vulkano::{
    device::Device, 
    image::SampleCount, 
    pipeline::{
        GraphicsPipeline, 
        graphics::{viewport::Viewport
        }
    }, 
    render_pass::RenderPass, 
    shader::ShaderModule
};

use crate::engine::{renderer::{pipeline_constructor::PipelineConstructor, shader_manager::ShaderManager}, scene::mesh::OsmiumVertex};

pub struct MaterialConfig {
    pub vs_path: String,
    pub fs_path: String
}

impl MaterialConfig {
    pub fn new() -> Self {
        Self {
            vs_path: "./shaders/vertex.glsl".into(),
            fs_path: "./shaders/fragment.glsl".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Material {
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,

    pipeline: Option<Arc<GraphicsPipeline>>
}

impl Material {
    pub fn init(
        material_config: &MaterialConfig,
        shader_manager: &ShaderManager
    ) -> Self {
        let vs: Arc<ShaderModule> = unsafe {
            shader_manager.create_shader(
                &material_config.vs_path,
                ShaderKind::Vertex,
            )
        };
        let fs: Arc<ShaderModule> = unsafe {
            shader_manager.create_shader(
                &material_config.fs_path,
                ShaderKind::Fragment,
            )
        };

        Self {
            vs, fs,
            pipeline: None
        }
    }

    pub fn recreate_pipeline(
        &mut self,
        device: Arc<Device>,
        render_pass: Arc<RenderPass>, 
        viewport: Viewport,
        samples: SampleCount,
        depth_enabled: bool
    ) {
        let pipeline = PipelineConstructor::get_pipeline::<OsmiumVertex>(
            device, 
            self.vs.clone(), self.fs.clone(), 
            render_pass, 
            viewport, 
            samples, 
            depth_enabled
        );

        self.pipeline = Some(pipeline);
    }

    pub fn get_pipeline(&self) -> Arc<GraphicsPipeline> {
        match &self.pipeline {
            Some(p) => p.clone(),
            None => panic!("Pipeline not yet created!")
        }
    }
}