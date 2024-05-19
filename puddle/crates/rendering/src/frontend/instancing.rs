use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::{Commands, Query, Res, Resource},
};
use components::Transform;
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
};

use crate::backend;

use super::types::InstanceData;

#[derive(Resource, Component)]
pub struct InstanceBuffer(pub Subbuffer<[InstanceData]>);

impl InstanceBuffer {
    pub fn new(
        memory_allocator: &backend::buffer::StandardMemoryAllocator,
        data: Vec<InstanceData>,
    ) -> Self {
        let buffer = Buffer::from_iter(
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
            data,
        )
        .unwrap();

        Self(buffer)
    }
}

#[derive(Resource, Component)]
pub struct InstancingTransforms(pub Vec<Transform>);

pub fn reload_instance_buffer(
    query: Query<(Entity, Option<&Transform>, Option<&InstancingTransforms>)>,
    allocator: Res<backend::buffer::StandardMemoryAllocator>,
    mut commands: Commands,
) {
    for (entity, transform, instances) in query.iter() {

        if transform.is_none() && instances.is_none() {
            continue;
        }


        let mut data = vec![];

        if let Some(transform) = transform {
            data.push((*transform).into());
        }

        if let Some(instances) = instances {
            for v in instances.0.iter() {
                data.push(InstanceData::from(*v));
            }
        }

        let buffer = InstanceBuffer::new(&allocator, data);
        commands.entity(entity).insert(buffer);
    }
}
