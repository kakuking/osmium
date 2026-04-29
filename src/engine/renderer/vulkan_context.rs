use std::sync::Arc;

use vulkano::{
    VulkanLibrary, 
    command_buffer::allocator::{
        StandardCommandBufferAllocator, 
        StandardCommandBufferAllocatorCreateInfo
    }, device::{
        Device, 
        DeviceCreateInfo, 
        DeviceExtensions, 
        Queue, 
        QueueCreateInfo, 
        QueueFlags, 
        physical::{
            PhysicalDevice, 
            PhysicalDeviceType
        }
    }, instance::{
        Instance, 
        InstanceCreateFlags, 
        InstanceCreateInfo
    }, memory::allocator::{
        StandardMemoryAllocator
    }, swapchain::Surface
};

use crate::engine::{
    renderer::config::RendererConfig, 
    window::window_manager::WindowManager
};

pub struct VulkanContext {
    pub instance: Arc<Instance>,
    pub physical_device: Arc<PhysicalDevice>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub memory_allocator: Arc<StandardMemoryAllocator>,
    pub command_buffer_allocator: StandardCommandBufferAllocator,
}

impl VulkanContext {
    pub fn create(
        window_manager: &mut WindowManager,
        config: &RendererConfig
    ) -> Self {
        let library = VulkanLibrary::new()
            .expect("No local vulkan lib or DLL");

        let mut enabled_layers: Vec<String> = Vec::new();

        if config.enable_validation {
            enabled_layers.push("VK_LAYER_KHRONOS_validation".to_string());
        }

        let instance: Arc<Instance> = Instance::new(
            library, 
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: window_manager.get_required_extensions(),
                enabled_layers,
                ..Default::default()
            },
        ).expect("Failed to create an instance");

        window_manager.create_surface(
            instance.clone()
        );

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let (physical_device, queue_family_index) = Self::select_physical_device(
            instance.clone(), 
            window_manager.get_surface(), 
            &device_extensions,
            &config
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

        let command_buffer_allocator = 
            StandardCommandBufferAllocator::new(
            device.clone(), 
            StandardCommandBufferAllocatorCreateInfo::default()
        );

        Self {
            instance,
            physical_device,
            device,
            queue,
            memory_allocator,
            command_buffer_allocator,
        }
    }

    pub fn get_memory_allocator(&self) -> Arc<StandardMemoryAllocator> {
        self.memory_allocator.clone()
    }

    pub fn get_device(&self) -> Arc<Device> {
        self.device.clone()
    }
    
    pub fn get_queue(&self) -> Arc<Queue> {
        self.queue.clone()
    }

    fn select_physical_device(
        instance: Arc<Instance>, 
        surface: Arc<Surface>, 
        device_extensions: &DeviceExtensions,
        config: &RendererConfig
    ) -> (Arc<PhysicalDevice>, u32) {
        let gpus: Vec<(Arc<PhysicalDevice>, u32)> = instance
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
            .collect();

            for preferred_gpu in &config.gpu_priority {
                for (device_gpu, queue) in &gpus {
                    if device_gpu.properties().device_type == *preferred_gpu {
                        return (device_gpu.clone(), *queue);
                    }
                }
            }

            gpus
                .into_iter()
                .min_by_key(|(p, _)| {
                match p.properties().device_type {
                    PhysicalDeviceType::DiscreteGpu => 0,
                    PhysicalDeviceType::IntegratedGpu => 1,
                    PhysicalDeviceType::VirtualGpu => 2,
                    PhysicalDeviceType::Cpu => 3,
                    _ => 4,
                }
            })
            .expect("No available devices")
    }
}