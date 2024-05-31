use std::sync::Arc;

use bevy_ecs::{component::Component, system::Resource};
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
};


#[derive(Component, Resource)]
pub struct IndexBuffer(pub Subbuffer<[u16]>);

impl IndexBuffer {
    pub fn new(
        memory_allocator: &crate::StandardMemoryAllocator,
        indices: Arc<[u16]>,
    ) -> Self {
        let index_buffer = Buffer::from_iter(
            memory_allocator.0.clone(),
            BufferCreateInfo {
                usage: BufferUsage::INDEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            indices.iter().cloned(),
        )
        .unwrap();

        Self(index_buffer)
    }
}

