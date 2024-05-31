use bevy_ecs::system::Resource;
use std::{ops::Deref, sync::Arc};
use vulkano::{instance, swapchain};

// first thing to to when loading the renderer is to create the API entry point
// we need it to get other things like the physical device (gpu)

#[derive(Clone, Resource)]
pub struct Instance(pub Arc<vulkano::instance::Instance>);

// creates a render instance from a EventLoop to ensure that its compatible
impl Instance {
    pub fn new(event_loop: &window::EventLoop) -> Self {
        let library = vulkano::VulkanLibrary::new().unwrap();

        // make sure the required extensions to render on a window are there
        let required_extensions = swapchain::Surface::required_extensions(&event_loop.0).unwrap();

        // create the render instance
        let instance = instance::Instance::new(
            library,
            instance::InstanceCreateInfo {
                flags: instance::InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: required_extensions,
                ..Default::default()
            },
        )
        .unwrap();

        Instance(instance)
    }
}

impl Into<Arc<vulkano::instance::Instance>> for Instance {
    fn into(self) -> Arc<vulkano::instance::Instance> {
        self.0
    }
}

impl Deref for Instance {
    type Target = Arc<vulkano::instance::Instance>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
