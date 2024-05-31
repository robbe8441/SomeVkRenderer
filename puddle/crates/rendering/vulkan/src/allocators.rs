use std::sync::Arc;

use bevy_ecs::system::{Commands, Resource};
use vulkano::{
    command_buffer::allocator::StandardCommandBufferAllocator,
    descriptor_set::allocator::StandardDescriptorSetAllocator, memory::allocator,
};

use super::Device;

#[allow(unused)]
#[derive(Resource)]
pub struct StandardMemoryAllocator(pub Arc<allocator::StandardMemoryAllocator>);

#[allow(unused)]
#[derive(Resource)]
pub struct CommandBufferAllocator(pub Arc<StandardCommandBufferAllocator>);

#[allow(unused)]
#[derive(Resource)]
pub struct DescriptorSetAllocator(pub Arc<StandardDescriptorSetAllocator>);

pub fn setup(commands: &mut Commands, render_device: &Device) {
    let memory_allocator = Arc::new(allocator::StandardMemoryAllocator::new_default(
        render_device.device.clone(),
    ));

    let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
        render_device.device.clone(),
        Default::default(),
    ));

    let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
        render_device.device.clone(),
        Default::default(),
    ));

    commands.insert_resource(StandardMemoryAllocator(memory_allocator));
    commands.insert_resource(CommandBufferAllocator(command_buffer_allocator));
    commands.insert_resource(DescriptorSetAllocator(descriptor_set_allocator));
}
