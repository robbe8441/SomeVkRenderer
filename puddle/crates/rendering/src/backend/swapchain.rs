use std::sync::Arc;

use bevy_ecs::system::{Commands, Res, Resource};
use vulkano::{
    image::{Image, ImageUsage},
    swapchain::{PresentMode, SwapchainCreateInfo},
};
use window::Window;

use super::{device::RenderDevice, surface::RenderSurface};

#[allow(unused)]
#[derive(Resource)]
pub struct Swapchain {
    pub swapchain: Arc<vulkano::swapchain::Swapchain>,
    pub images: Vec<Arc<Image>>,
    pub recreate_swapchain: bool,
}

pub fn create_swapchain(
    window: Res<Window>,
    surface: Res<RenderSurface>,
    render_device: Res<RenderDevice>,
    mut commands: Commands,
) {
    let (swapchain, images) = {
        let surface_capabilities = render_device
            .device
            .physical_device()
            .surface_capabilities(&surface.0, Default::default())
            .unwrap();
        let image_format = render_device
            .device
            .physical_device()
            .surface_formats(&surface.0, Default::default())
            .unwrap()[0]
            .0;

        vulkano::swapchain::Swapchain::new(
            render_device.device.clone(),
            surface.0.clone(),
            SwapchainCreateInfo {
                min_image_count: surface_capabilities.min_image_count.max(2),
                image_format,
                image_extent: window.0.inner_size().into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                present_mode: PresentMode::Immediate,
                composite_alpha: surface_capabilities
                    .supported_composite_alpha
                    .into_iter()
                    .next()
                    .unwrap(),
                ..Default::default()
            },
        )
        .unwrap()
    };

    commands.insert_resource(Swapchain {
        swapchain,
        images,
        recreate_swapchain: false,
    });
}
