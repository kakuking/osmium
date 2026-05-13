use std::{fs, path::{Path, PathBuf}, sync::Arc};

use shaderc::{IncludeType, ResolvedInclude, ShaderKind};
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

        let shader_path = PathBuf::from(filepath);
        let shader_dir = shader_path.parent()
            .unwrap_or(Path::new("."))
            .to_path_buf();

        let mut options = shaderc::CompileOptions::new()
            .unwrap();

        options.set_optimization_level(
            shaderc::OptimizationLevel::Zero
        );

        options.set_include_callback(move |requested_source, include_type, requesting_source, _depth| {
            let include_path = match include_type {
                IncludeType::Relative => {
                    let base = Path::new(requesting_source)
                        .parent()
                        .unwrap_or(&shader_dir);

                    base.join(requested_source)
                }
                IncludeType::Standard => {
                    PathBuf::from("./shaders/include").join(requested_source)
                }
            };

            let content = fs::read_to_string(&include_path)
                .map_err(|e| format!("Failed to include {:?}: {}", include_path, e))?;

            Ok(ResolvedInclude {
                resolved_name: include_path.to_string_lossy().into_owned(),
                content,
            })
        });

        let artifact = self.compiler.compile_into_spirv(
            &src, 
            kind, 
            filepath, 
            "main", 
            Some(&options),
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