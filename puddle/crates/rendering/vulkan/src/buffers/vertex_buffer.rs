use std::sync::Arc;

use bevy_ecs::{component::Component, system::Resource};
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
};

#[derive(Component, Resource)]
pub struct VertexBuffer(pub Subbuffer<[utils::Vertex]>);

impl VertexBuffer {
    pub fn new(
        memory_allocator: &crate::StandardMemoryAllocator,
        vertices: Arc<[utils::Vertex]>,
    ) -> Self {
        let vertex_buffer = Buffer::from_iter(
            memory_allocator.0.clone(),
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            vertices.iter().cloned(),
        )
        .unwrap();

        Self(vertex_buffer)
    }
}
