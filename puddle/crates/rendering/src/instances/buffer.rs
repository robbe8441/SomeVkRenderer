use std::sync::Arc;

use vulkano::{
    buffer::{allocator::SubbufferAllocator, BufferContents, BufferUsage, Subbuffer},
    memory::allocator::{MemoryTypeFilter, StandardMemoryAllocator},
};

pub struct BufferAllocator {
    pub allocator: Arc<SubbufferAllocator>,
}

pub struct BufferDesc {
    pub allocator: Arc<StandardMemoryAllocator>,
    pub usage: BufferUsage,
    pub memory_type_filter: MemoryTypeFilter,
}

impl BufferAllocator {
    pub fn new(desc: &BufferDesc) -> Self {
        let allocator = Arc::new(SubbufferAllocator::new(
            desc.allocator.clone(),
            vulkano::buffer::allocator::SubbufferAllocatorCreateInfo {
                buffer_usage: desc.usage,
                memory_type_filter: desc.memory_type_filter,
                ..Default::default()
            },
        ));

        Self { allocator }
    }

    #[inline(always)]
    pub fn write<T>(&self, data: T) -> Subbuffer<T>
    where
        T: BufferContents,
    {
        let subbuffer = self.allocator.allocate_sized().unwrap();
        *subbuffer.write().unwrap() = data;
        subbuffer
    }
}
