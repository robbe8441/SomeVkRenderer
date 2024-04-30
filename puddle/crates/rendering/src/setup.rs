use std::{sync::Arc, time::Instant};

use vulkano::{
    buffer::BufferUsage,
    command_buffer::allocator::StandardCommandBufferAllocator,
    descriptor_set::allocator::StandardDescriptorSetAllocator,
    device::{
        self, physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Queue,
        QueueCreateInfo, QueueFlags,
    },
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::{MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::GraphicsPipeline,
    swapchain,
    sync::{self, GpuFuture},
    Version, VulkanLibrary,
};

use crate::instances;

pub struct ForwardRenderer {
    pub device: Arc<Device>,
    pub instance: Arc<Instance>,
    pub queue: Arc<Queue>,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    pub previous_frame_end: Option<Box<dyn GpuFuture>>,
}

pub struct ExampleScene {
    pub pipeline: Arc<GraphicsPipeline>,
    pub rotation_start: Instant,
    pub uniform_buffer: instances::BufferAllocator,
    pub descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
}

pub fn init(app: &mut application::Application) {
    let window = &app
        .resources
        .get::<window::PuddleWindow>()
        .unwrap().clone();


    let library = VulkanLibrary::new().unwrap();

    let required_extensions = swapchain::Surface::required_extensions(&window.window.clone());

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


    let mut device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let surface = swapchain::Surface::from_window(instance.clone(), window.window.clone()).unwrap();

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

    // When creating the swapchain, we only created plain images. To use them as an attachment for
    // rendering, we must wrap then in an image view.
    //
    // Since we need to draw to multiple images, we are going to create a different image view for
    // each image.

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


    let previous_frame_end = Some(sync::now(device.clone()).boxed());

    let renderer = ForwardRenderer {
        command_buffer_allocator,
        device: device.clone(),
        instance,
        previous_frame_end,
        queue,
    };

    let surface = instances::Surface::from_window(&renderer, &window);

    let uniform_buffer = instances::BufferAllocator::new(&instances::BufferDesc {
        allocator: memory_allocator.clone(),
        memory_type_filter: MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
        usage: BufferUsage::UNIFORM_BUFFER,
    });

    let rotation_start = Instant::now();


    let pipeline = instances::create_pipeline(device.clone(), surface.swapchain.clone(), vs, fs);

    app.resources.insert(ExampleScene {
        pipeline,
        rotation_start,
        uniform_buffer,
        descriptor_set_allocator,
    });

    app.resources.insert(renderer);

    let entt = app.world.push(());

    app.world.entry(entt).and_then(|mut entry| {
        entry.add_component(surface);
        Some(())
    });
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
