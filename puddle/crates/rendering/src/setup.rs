use std::{sync::Arc, time::Instant};

use vulkano::{
    buffer::BufferUsage,
    command_buffer::allocator::StandardCommandBufferAllocator,
    descriptor_set::allocator::StandardDescriptorSetAllocator,
    device::{
        self, physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Queue,
        QueueCreateInfo, QueueFlags,
    },
    image::{view::ImageView, Image, ImageUsage},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::{MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{graphics::viewport::Viewport, GraphicsPipeline},
    swapchain::{Surface, Swapchain, SwapchainCreateInfo},
    sync::{self, GpuFuture},
    Version, VulkanLibrary,
};

use crate::instances;

pub struct ForwardRenderer {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    pub previous_frame_end: Option<Box<dyn GpuFuture>>,
}

pub struct RenderSurface {
    pub recreate_swapchain: bool,
    pub attachment_image_views: Vec<Arc<ImageView>>,
    pub viewport: Viewport,
    pub swapchain: Arc<Swapchain>,
}

pub struct ExampleSchene {
    pub pipeline: Arc<GraphicsPipeline>,
    pub rotation_start: Instant,
    pub uniform_buffer: instances::BufferAllocator,
    pub descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
}

pub fn init(app: &mut application::Application) {
    let window = &app
        .resources
        .get::<window::PuddleWindow>()
        .unwrap()
        .window
        .clone();

    let library = VulkanLibrary::new().unwrap();

    let required_extensions = Surface::required_extensions(&window.clone());

    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            // Enable enumerating devices that use non-conformant Vulkan implementations.
            // (e.g. MoltenVK)
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            enabled_extensions: required_extensions.unwrap(),
            ..Default::default()
        },
    )
    .unwrap();

    let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();

    let mut device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    // filter out devices that we dont want
    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        .unwrap()
        .filter(|p| {
            p.api_version() >= Version::V1_3 || p.supported_extensions().khr_dynamic_rendering
        })
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.intersects(QueueFlags::GRAPHICS)
                        && p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|i| (p, i as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .expect("no suitable physical device found");

    println!(
        "Using device: {} (type: {:?})",
        physical_device.properties().device_name,
        physical_device.properties().device_type,
    );

    // If the selected device doesn't have Vulkan 1.3 available, then we need to enable the
    // `khr_dynamic_rendering` extension manually. This extension became a core part of Vulkan
    // in version 1.3 and later, so it's always available then and it does not need to be enabled.
    // We can be sure that this extension will be available on the selected physical device,
    // because we filtered out unsuitable devices in the device selection code above.
    if physical_device.api_version() < Version::V1_3 {
        device_extensions.khr_dynamic_rendering = true;
    }

    // Now initializing the device. This is probably the most important object of Vulkan.
    //
    // An iterator of created queues is returned by the function alongside the device.
    let (device, mut queues) = Device::new(
        // Which physical device to connect to.
        physical_device,
        DeviceCreateInfo {
            // The list of queues that we are going to use. Here we only use one queue, from the
            // previously chosen queue family.
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],

            enabled_extensions: device_extensions,

            enabled_features: device::DeviceFeatures {
                dynamic_rendering: true,
                ..device::DeviceFeatures::empty()
            },

            ..Default::default()
        },
    )
    .unwrap();

    // Since we can request multiple queues, the `queues` variable is in fact an iterator. We only
    // use one queue in this example, so we just retrieve the first and only element of the
    // iterator.
    let queue = queues.next().unwrap();

    // Before we can draw on the surface, we have to create what is called a swapchain. Creating a
    // swapchain allocates the color buffers that will contain the image that will ultimately be
    // visible on the screen. These images are returned alongside the swapchain.
    let (swapchain, images) = {
        // Querying the capabilities of the surface. When we create the swapchain we can only pass
        // values that are allowed by the capabilities.
        let surface_capabilities = device
            .physical_device()
            .surface_capabilities(&surface, Default::default())
            .unwrap();

        // Choosing the internal format that the images will have.
        let image_format = device
            .physical_device()
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0;

        // Please take a look at the docs for the meaning of the parameters we didn't mention.
        Swapchain::new(
            device.clone(),
            surface,
            SwapchainCreateInfo {
                // Some drivers report an `min_image_count` of 1, but fullscreen mode requires at
                // least 2. Therefore we must ensure the count is at least 2, otherwise the program
                // would crash when entering fullscreen mode on those drivers.
                min_image_count: surface_capabilities.min_image_count.max(2),

                image_format,

                // The size of the window, only used to initially setup the swapchain.
                //
                // NOTE:
                // On some drivers the swapchain extent is specified by
                // `surface_capabilities.current_extent` and the swapchain size must use this
                // extent. This extent is always the same as the window size.
                //
                // However, other drivers don't specify a value, i.e.
                // `surface_capabilities.current_extent` is `None`. These drivers will allow
                // anything, but the only sensible value is the window size.
                //
                // Both of these cases need the swapchain to use the window size, so we just
                // use that.
                image_extent: window.inner_size().into(),

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

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
        device.clone(),
        Default::default(),
    ));

    // We now create a buffer that will store the shape of our triangle. We use `#[repr(C)]` here
    // to force rustc to use a defined layout for our data, as the default representation has *no
    // guarantees*.

    // Dynamic viewports allow us to recreate just the viewport when the window is resized.
    // Otherwise we would have to recreate the whole pipeline.
    let mut viewport = Viewport {
        offset: [0.0, 0.0],
        extent: [0.0, 0.0],
        depth_range: 0.0..=1.0,
    };

    // When creating the swapchain, we only created plain images. To use them as an attachment for
    // rendering, we must wrap then in an image view.
    //
    // Since we need to draw to multiple images, we are going to create a different image view for
    // each image.
    let attachment_image_views = window_size_dependent_setup(&images, &mut viewport);

    // Before we can start creating and recording command buffers, we need a way of allocating
    // them. Vulkano provides a command buffer allocator, which manages raw Vulkan command pools
    // underneath and provides a safe interface for them.
    let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
        device.clone(),
        Default::default(),
    ));

    let vs = vs::load(device.clone())
        .unwrap()
        .entry_point("main")
        .unwrap();
    let fs = fs::load(device.clone())
        .unwrap()
        .entry_point("main")
        .unwrap();

    let pipeline = instances::create_pipeline(device.clone(), swapchain.clone(), vs, fs);

    // Initialization is finally finished!

    // In some situations, the swapchain will become invalid by itself. This includes for example
    // when the window is resized (as the images of the swapchain will no longer match the
    // window's) or, on Android, when the application went to the background and goes back to the
    // foreground.
    //
    // In this situation, acquiring a swapchain image or presenting it will return an error.
    // Rendering to an image of that swapchain will not produce any error, but may or may not work.
    // To continue rendering, we need to recreate the swapchain by creating a new swapchain. Here,
    // we remember that we need to do this for the next loop iteration.
    let recreate_swapchain = false;

    // In the loop below we are going to submit commands to the GPU. Submitting a command produces
    // an object that implements the `GpuFuture` trait, which holds the resources for as long as
    // they are in use by the GPU.
    //
    // Destroying the `GpuFuture` blocks until the GPU is finished executing it. In order to avoid
    // that, we store the submission of the previous frame here.
    let previous_frame_end = Some(sync::now(device.clone()).boxed());

    // let vertex_buffer = Buffer::from_iter(
    //     memory_allocator,
    //     BufferCreateInfo {
    //         usage: BufferUsage::VERTEX_BUFFER,
    //         ..Default::default()
    //     },
    //     AllocationCreateInfo {
    //         memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
    //             | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
    //             ..Default::default()
    //     },
    //     vertices,
    //     )
    //     .unwrap();
    //

    // vulkano::buffer::allocator::SubbufferAllocatorCreateInfo {
    //     buffer_usage: BufferUsage::UNIFORM_BUFFER,
    //     memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
    //         | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
    //         ..Default::default()
    // },

    let uniform_buffer = instances::BufferAllocator::new(&instances::BufferDesc {
        allocator: memory_allocator.clone(),
        memory_type_filter: MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
        usage: BufferUsage::UNIFORM_BUFFER,
    });

    let rotation_start = Instant::now();

    app.resources.insert(ForwardRenderer {
        device,
        queue,
        command_buffer_allocator,
        previous_frame_end,
    });

    let entt = app.world.push(());

    app.world.entry(entt).and_then(|mut entry| {
        entry.add_component(RenderSurface {
            swapchain,
            viewport,
            attachment_image_views,
            recreate_swapchain,
        });
        Some(())
    });

    app.resources.insert(ExampleSchene {
        pipeline,
        rotation_start,
        uniform_buffer,
        descriptor_set_allocator,
    });
}

/// This function is called once during initialization, then again whenever the window is resized.
fn window_size_dependent_setup(
    images: &[Arc<Image>],
    viewport: &mut Viewport,
) -> Vec<Arc<ImageView>> {
    let extent = images[0].extent();
    viewport.extent = [extent[0] as f32, extent[1] as f32];

    images
        .iter()
        .map(|image| ImageView::new_default(image.clone()).unwrap())
        .collect::<Vec<_>>()
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 450

            const vec2[6] POSITIONS = {
                vec2(-1.0, -1.0),
                vec2( 1.0,  1.0),
                vec2(-1.0,  1.0),
                vec2(-1.0, -1.0),
                vec2( 1.0, -1.0),
                vec2( 1.0,  1.0),
            };

            const vec2[6] TEX_COORDS = {
                vec2(0.0, 1.0),
                vec2(1.0, 0.0),
                vec2(0.0, 0.0),
                vec2(0.0, 1.0),
                vec2(1.0, 1.0),
                vec2(1.0, 0.0),
            };

            layout(location = 0) out vec2 f_tex_coords;

            void main() {
                gl_Position = vec4(POSITIONS[gl_VertexIndex], 0.0, 1.0);
                f_tex_coords = TEX_COORDS[gl_VertexIndex];
            }
        ",
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/fragment.glsl"
    }
}
