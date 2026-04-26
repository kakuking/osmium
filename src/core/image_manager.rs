use std::sync::Arc;

use vulkano::{format::Format, image::{Image, ImageCreateInfo, ImageType, ImageUsage, view::ImageView}, memory::allocator::{AllocationCreateInfo, MemoryAllocator, MemoryTypeFilter}};

pub struct ImageManager {
    pub memory_allocator: Arc<dyn MemoryAllocator>,
}

impl ImageManager {
    pub fn init(memory_allocator: Arc<dyn MemoryAllocator>) -> Self {
        Self {
            memory_allocator
        }
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
}