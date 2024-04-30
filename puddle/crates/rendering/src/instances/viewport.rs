use std::sync::Arc;

use vulkano::{
    image::{view::ImageView, Image, ImageUsage},
    pipeline::graphics::viewport::Viewport,
    swapchain::{self, Swapchain, SwapchainCreateInfo},
};

pub struct Surface {
    pub recreate_swapchain: bool,
    pub attachment_image_views: Vec<Arc<ImageView>>,
    pub images: Vec<Arc<Image>>,
    pub viewport: Viewport,
    pub swapchain: Arc<Swapchain>,
}

impl Surface {

    pub fn from_window(renderer: &crate::setup::ForwardRenderer, window: &window::PuddleWindow) -> Self {
        let surface =
            swapchain::Surface::from_window(renderer.instance.clone(), window.window.clone())
                .unwrap();

        let (swapchain, images) = {
            // Querying the capabilities of the surface. When we create the swapchain we can only pass
            // values that are allowed by the capabilities.
            let surface_capabilities = renderer
                .device
                .physical_device()
                .surface_capabilities(&surface, Default::default())
                .unwrap();

            // Choosing the internal format that the images will have.
            let image_format = renderer
                .device
                .physical_device()
                .surface_formats(&surface, Default::default())
                .unwrap()[0]
                .0;

            // Please take a look at the docs for the meaning of the parameters we didn't mention.
            Swapchain::new(
                renderer.device.clone(),
                surface,
                SwapchainCreateInfo {
                    // Some drivers report an `min_image_count` of 1, but fullscreen mode requires at
                    // least 2. Therefore we must ensure the count is at least 2, otherwise the program
                    // would crash when entering fullscreen mode on those drivers.
                    min_image_count: surface_capabilities.min_image_count.max(2),

                    image_format,

                    image_extent: window.window.inner_size().into(),

                    image_usage: ImageUsage::COLOR_ATTACHMENT,

                    present_mode: vulkano::swapchain::PresentMode::Immediate,

                    // The alpha mode indicates how the alpha value of the final image will behave. For
                    // example, you can choose whether the window will be opaque or transparent.
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

        let mut viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [0.0, 0.0],
            depth_range: 0.0..=1.0,
        };

        let attachment_image_views = window_size_dependent_setup(&images, &mut viewport);

        Self {
            recreate_swapchain: false,
            images,
            attachment_image_views,
            viewport,
            swapchain,
        }
    }

    #[inline(always)]
    pub fn reload_images(&mut self) {
        self.attachment_image_views = window_size_dependent_setup(&self.images, &mut self.viewport);
    }

    pub fn reload_swapchain(&mut self, image_extent: [u32; 2]) {
        let (new_swapchain, new_images) = self
            .swapchain
            .recreate(SwapchainCreateInfo {
                image_extent,
                ..self.swapchain.create_info()
            })
            .expect("failed to recreate swapchain");

        self.swapchain = new_swapchain;

        // Now that we have new swapchain images, we must create new image views from
        // them as well.
        self.attachment_image_views = window_size_dependent_setup(&new_images, &mut self.viewport);

        self.recreate_swapchain = false;
    }
}

/// This function is called once during initialization, then again whenever the window is resized.
fn window_size_dependent_setup(
    images: &Vec<Arc<Image>>,
    viewport: &mut Viewport,
) -> Vec<Arc<ImageView>> {
    let extent = images[0].extent();
    viewport.extent = [extent[0] as f32, extent[1] as f32];

    images
        .iter()
        .map(|image| ImageView::new_default(image.clone()).unwrap())
        .collect::<Vec<_>>()
}
