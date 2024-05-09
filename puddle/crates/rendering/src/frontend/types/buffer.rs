use bevy_ecs::component::Component;
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
};

use super::Vertex3D;
use crate::backend;

#[derive(Component)]
pub struct VertexBuffer(pub Subbuffer<[Vertex3D]>);

impl VertexBuffer {
    pub fn new(
        memory_allocator: &backend::buffer::StandardMemoryAllocator,
        vertices: &Vec<Vertex3D>,
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
            vertices.clone(),
        )
        .unwrap();

        Self(vertex_buffer)
    }
}
