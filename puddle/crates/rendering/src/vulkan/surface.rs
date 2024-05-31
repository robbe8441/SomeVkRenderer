use std::sync::Arc;

use bevy_ecs::system::Resource;
use vulkano::swapchain;

#[derive(Resource)]
pub struct Surface(pub Arc<swapchain::Surface>);

impl Surface {
    pub fn new(instance: &super::Instance, window: &window::Window) -> Self {
        Surface(swapchain::Surface::from_window(instance.0.clone(), window.0.clone()).unwrap())
    }
}
