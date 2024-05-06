use std::sync::Arc;

use bevy_ecs::system::{Commands, NonSend, Resource};
use vulkano::{
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    swapchain::Surface,
    VulkanLibrary,
};

#[derive(Resource)]
pub struct RenderInstance(pub Arc<Instance>);

pub fn create_instance(mut commands: Commands, event_loop: NonSend<window::EventLoop>) {
    let library = VulkanLibrary::new().unwrap();

    let required_extensions = Surface::required_extensions(&event_loop.0).unwrap();

    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    )
    .unwrap();
    commands.insert_resource(RenderInstance(instance));
}
