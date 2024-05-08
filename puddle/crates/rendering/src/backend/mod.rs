use application::{Application, PreStartup, PreUpdate};
use bevy_ecs::schedule::IntoSystemConfigs;

pub mod buffer;
pub mod device;
pub mod instance;
pub mod pipeline;
pub mod surface;
pub mod swapchain;

pub fn init(app: &mut Application) {
    app.add_systems(
        PreStartup,
        (
            instance::create_instance,
            surface::create_surface,
            device::create_device,
            swapchain::create_swapchain,
            buffer::setup_memory_allocators,
        )
            .chain(),
    );

    app.add_systems(PreUpdate, swapchain::on_window_resize);
}
