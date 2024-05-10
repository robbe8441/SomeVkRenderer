use std::sync::Arc;

use bevy_ecs::{component::Component, system::Resource};
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer}, descriptor_set::DescriptorSet, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter}
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

#[derive(Component, Resource)]
pub struct VoxelBuffer {
    pub buffer: Subbuffer<[u8]>,
    pub size: [u32; 3],
}

#[derive(Component)]
pub struct VoxelDescriptorSet(pub Arc<DescriptorSet>);


impl VoxelBuffer {
    pub fn new(
        memory_allocator: &backend::buffer::StandardMemoryAllocator,
        voxels: Vec<u8>,
        size: [u32; 3],
    ) -> Self {
        let upload_buffer = Buffer::from_iter(
            memory_allocator.0.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            voxels,
        )
        .unwrap();

        Self {
            buffer: upload_buffer,
            size,
        }
    }
}
