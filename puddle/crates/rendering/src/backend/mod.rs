use application::{Application, Startup};
use bevy_ecs::schedule::IntoSystemConfigs;

pub mod buffer;
pub mod device;
pub mod instance;
pub mod surface;
pub mod swapchain;

pub fn init(app: &mut Application) {
    app.add_systems(
        Startup,
        (
            instance::create_instance,
            surface::create_surface,
            device::create_device,
            swapchain::create_swapchain,
            buffer::setup_memory_allocators,
        )
            .chain(),
    );
}
