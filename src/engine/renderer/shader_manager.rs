use std::{fs, sync::Arc};

use shaderc::ShaderKind;
use vulkano::{
    device::Device, 
    shader::{
        ShaderModule, 
        ShaderModuleCreateInfo
    }
};

pub struct ShaderManager {
    device: Arc<Device>,
    compiler: shaderc::Compiler
}

impl ShaderManager {
    pub fn new(device: Arc<Device>) -> Self {
        let compiler = shaderc::Compiler::new()
            .expect("Failed to create shader compiler");

        Self {
            device,
            compiler
        }
    }

    unsafe fn compile_glsl(&self, filepath: &str, kind: shaderc::ShaderKind) -> Arc<ShaderModule> {
        let src = fs::read_to_string(filepath)
            .expect("Failed to read glsl file");

        let artifact = self.compiler.compile_into_spirv(
            &src, 
            kind, 
            filepath, 
            "main", 
            None
        ).expect("Failed to compile shader");

        unsafe {
            ShaderModule::new(
                self.device.clone(), 
                ShaderModuleCreateInfo::new(
                    artifact.as_binary()
                )
            )
        }.expect("Could not create shader module")
    }

    pub unsafe fn 
    create_shader(
        &self, 
        filepath: &str,
        kind: ShaderKind
    ) -> Arc<ShaderModule> {
        unsafe {
            self.compile_glsl(
                filepath, 
                kind
            )
        }
    }
}