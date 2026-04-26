use std::{fmt::Debug, sync::Arc};

use vulkano::{buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer}, memory::allocator::{AllocationCreateInfo, MemoryAllocator, MemoryTypeFilter}};

pub struct BufferManager {
    pub memory_allocator: Arc<dyn MemoryAllocator>,
}

impl BufferManager {
    pub fn init(memory_allocator: Arc<dyn MemoryAllocator>) -> Self {
        Self {
            memory_allocator
        }
    }

    pub fn create_buffer_from_data<T: BufferContents + Debug>(
        &mut self, 
        data: T,
        usage: Option<BufferUsage>,
        memory_type_filter: Option<MemoryTypeFilter>
    ) -> Subbuffer<T> {
        let usage = usage.unwrap_or(BufferUsage::UNIFORM_BUFFER);
        
        let memory_type_filter = memory_type_filter.unwrap_or(
            MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE
        );

        let buffer = Buffer::from_data(
            self.memory_allocator.clone(), 
            BufferCreateInfo {
                usage,
                ..Default::default()
            }
            , 
            AllocationCreateInfo {
                memory_type_filter,
                ..Default::default()
            },
            data
        ).expect("Could not create buffer from data");

        buffer
    }

    pub fn create_buffer_from_iter<T, I>(
        &mut self, 
        iter: I,
        usage: Option<BufferUsage>,
        memory_type_filter: Option<MemoryTypeFilter>
    ) -> Subbuffer<[T]> 
        where
        T: BufferContents,
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        let usage = usage.unwrap_or(BufferUsage::UNIFORM_BUFFER);
        
        let memory_type_filter = memory_type_filter.unwrap_or(
            MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE
        );

        let buffer = Buffer::from_iter(
            self.memory_allocator.clone(), 
            BufferCreateInfo {
                usage,
                ..Default::default()
            }
            , 
            AllocationCreateInfo {
                memory_type_filter,
                ..Default::default()
            },
            iter
        ).expect("Could not create buffer from data");

        buffer
    }
}