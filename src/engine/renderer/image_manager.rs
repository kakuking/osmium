use std::{path::Path, sync::Arc};

use vulkano::{
    buffer::{
        Buffer, 
        BufferCreateInfo, 
        BufferUsage
    }, 
    command_buffer::{
        AutoCommandBufferBuilder, 
        CommandBufferUsage, 
        CopyBufferToImageInfo, 
        PrimaryCommandBufferAbstract, 
        allocator::StandardCommandBufferAllocator
    }, 
    device::Queue, 
    format::Format, 
    image::{
        Image, ImageCreateInfo, 
        ImageType, ImageUsage, 
        sampler::{
            Filter, Sampler, 
            SamplerAddressMode, SamplerCreateInfo
        }, 
        view::ImageView
    }, memory::allocator::{
        AllocationCreateInfo, 
        MemoryAllocator, 
        MemoryTypeFilter
    }, sync::GpuFuture
};

pub struct Texture {
    pub image: Arc<Image>,
    pub view: Arc<ImageView>,
    pub sampler: Arc<Sampler>,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone)]
pub struct DefaultTextures {
    pub white: Arc<Texture>,
    pub black: Arc<Texture>,
    pub gray: Arc<Texture>,
    pub flat_normal: Arc<Texture>,
}

impl DefaultTextures {
    pub fn new(
        memory_allocator: Arc<dyn MemoryAllocator>,
        command_buffer_allocator: &StandardCommandBufferAllocator,
        queue: Arc<Queue>,
    ) -> Self {
        Self {
            white: ImageManager::create_1x1_texture(
                memory_allocator.clone(),
                [255, 255, 255, 255],
                command_buffer_allocator,
                queue.clone(),
            ),
            black: ImageManager::create_1x1_texture(
                memory_allocator.clone(),
                [0, 0, 0, 255],
                command_buffer_allocator,
                queue.clone(),
            ),
            gray: ImageManager::create_1x1_texture(
                memory_allocator.clone(),
                [128, 128, 128, 255],
                command_buffer_allocator,
                queue.clone(),
            ),
            flat_normal: ImageManager::create_1x1_texture(
                memory_allocator,
                [128, 128, 255, 255],
                command_buffer_allocator,
                queue,
            ),
        }
    }
}

pub struct ImageManager {
    pub memory_allocator: Arc<dyn MemoryAllocator>,
    pub default_textures: DefaultTextures
}

impl ImageManager {
    pub fn init(
        memory_allocator: Arc<dyn MemoryAllocator>,
        command_buffer_allocator: &StandardCommandBufferAllocator,
        queue: Arc<Queue>,
    ) -> Self {
        Self {
            default_textures: DefaultTextures::new(
                memory_allocator.clone(), 
                command_buffer_allocator, 
                queue
            ),
            memory_allocator,
        }
    }

    pub fn create_1x1_texture(
        memory_allocator: Arc<dyn MemoryAllocator>,
        rgba: [u8; 4],
        command_buffer_allocator: &StandardCommandBufferAllocator,
        queue: Arc<Queue>,
    ) -> Arc<Texture> {
        let staging_buffer = Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            rgba,
        )
        .expect("Failed to create default texture staging buffer");

        let image = Image::new(
            memory_allocator.clone(),
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::R8G8B8A8_UNORM,
                extent: [1, 1, 1],
                usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
        )
        .expect("Failed to create default texture image");

        let mut builder = AutoCommandBufferBuilder::primary(
            command_buffer_allocator,
            queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
                staging_buffer,
                image.clone(),
            ))
            .unwrap();

        builder
            .build()
            .unwrap()
            .execute(queue.clone())
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();

        let view = ImageView::new_default(image.clone()).unwrap();

        let sampler = Sampler::new(
            queue.device().clone(),
            SamplerCreateInfo {
                mag_filter: Filter::Linear,
                min_filter: Filter::Linear,
                address_mode: [
                    SamplerAddressMode::Repeat,
                    SamplerAddressMode::Repeat,
                    SamplerAddressMode::Repeat,
                ],
                ..Default::default()
            },
        )
        .unwrap();

        Arc::new(Texture {
            image,
            view,
            sampler,
            width: 1,
            height: 1,
        })
    }

    pub fn create_image(
        &self,
        extent: [u32; 3],
        image_type: Option<ImageType>,
        format: Option<Format>,
        usage: Option<ImageUsage>,
        memory_type_filter: Option<MemoryTypeFilter>
    ) -> Arc<Image> {
        let image_type = image_type.unwrap_or(
            ImageType::Dim2d
        );
        let format = format.unwrap_or(
            Format::R8G8B8A8_UNORM
        );
        let usage = usage.unwrap_or(
            ImageUsage::TRANSFER_DST | ImageUsage::TRANSFER_SRC
        );

        let memory_type_filter = memory_type_filter.unwrap_or(
            MemoryTypeFilter::PREFER_DEVICE
        );

        let image = Image::new(
            self.memory_allocator.clone(),
            ImageCreateInfo {
                image_type,
                format,
                extent,
                usage,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter,
                ..Default::default()
            }
        ).expect("Could not create image");

        image
    }

    pub fn get_image_view(image: Arc<Image>) -> Arc<ImageView> {
        ImageView::new_default(image.clone()).unwrap()
    }

    pub fn load_texture<P: AsRef<Path>>(
        &self, 
        path: P,
        command_buffer_allocator: &StandardCommandBufferAllocator,
        queue: Arc<Queue>,
    ) -> Arc<Texture> {
        let img = image::open(&path)
            .expect("Failed to open texture")
            .to_rgba8();

        let (width, height) = img.dimensions();
        let pixels = img.into_raw();

        let staging_buffer = Buffer::from_iter(
            self.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            pixels,
        )
        .expect("Failed to create texture staging buffer");

        let image = Image::new(
            self.memory_allocator.clone(),
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::R8G8B8A8_UNORM,
                extent: [width, height, 1],
                usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
        )
        .expect("Failed to create texture image");

        let mut builder = AutoCommandBufferBuilder::primary(
            command_buffer_allocator,
            queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .expect("Failed to create texture upload command buffer");

        builder
            .copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
                staging_buffer,
                image.clone(),
            ))
            .expect("Failed to copy texture buffer to image");


        builder
            .build()
            .expect("Failed to build texture upload command buffer")
            .execute(queue.clone())
            .expect("Failed to execute texture upload command buffer")
            .then_signal_fence_and_flush()
            .expect("Failed to flush texture upload")
            .wait(None)
            .expect("Failed to wait for texture upload");

        let view = ImageView::new_default(image.clone())
            .expect("Failed to create texture image view");

        let sampler = Sampler::new(
            queue.device().clone(),
            SamplerCreateInfo {
                mag_filter: Filter::Linear,
                min_filter: Filter::Linear,
                address_mode: [
                    SamplerAddressMode::Repeat,
                    SamplerAddressMode::Repeat,
                    SamplerAddressMode::Repeat,
                ],
                ..Default::default()
            },
        )
        .expect("Failed to create texture sampler");

        Arc::new(Texture {
            image,
            view,
            sampler,
            width,
            height,
        })
    }
}