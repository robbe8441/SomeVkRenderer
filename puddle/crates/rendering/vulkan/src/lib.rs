mod allocators;
mod buffers;
mod device;
mod instance;
mod pipeline;
mod render_context;
mod surface;
mod swapchain;

use std::time::Instant;

use application::log::info;
use bevy_ecs::{
    system::{Commands, NonSend, Res, ResMut},
    world::World,
};

pub use allocators::*;
pub use buffers::*;
pub use device::*;
pub use instance::*;
pub use pipeline::*;
pub use render_context::*;
pub use surface::*;
pub use swapchain::*;

pub use vulkano::single_pass_renderpass;
pub use vulkano::{
    command_buffer::RenderPassBeginInfo,
    image::view::ImageView,
    pipeline::graphics::subpass::PipelineSubpassType,
    render_pass::{Framebuffer, FramebufferCreateInfo},
};

pub fn init(
    mut commands: Commands,
    event_loop: NonSend<window::EventLoop>,
    window: Res<window::Window>,
) {
    let start = Instant::now();

    let instance = Instance::new(&event_loop);
    let surface = Surface::new(&instance, &window);
    let device = Device::new(&instance, &surface);
    let swapchain = Swapchain::new(&device, &surface, window.0.inner_size());

    info!("took : {}s to load renderer", start.elapsed().as_secs_f64());

    allocators::setup(&mut commands, &device);

    commands.insert_resource(instance);
    commands.insert_resource(surface);
    commands.insert_resource(device);
    commands.insert_resource(swapchain);

    commands.add(|world: &mut World| {
        let device = world.get_resource::<Device>().unwrap();
        let render_context = RenderContext::new(&device);

        world.insert_non_send_resource(render_context);
    });
}

pub fn recreate_swapchain_system(window: Res<window::Window>, mut swapchain: ResMut<Swapchain>) {
    if swapchain.recreate_swapchain {
        swapchain.recreate(window.0.inner_size());
    }
}
