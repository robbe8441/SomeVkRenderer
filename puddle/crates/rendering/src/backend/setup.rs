use super::*;

pub fn init(app: &mut application::Application) -> Renderer {
    let window = app
        .resources
        .get::<window::PuddleWindow>()
        .expect("failed to load window")
        .window
        .clone();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });

    let window = app
        .resources
        .get_mut::<window::PuddleWindow>()
        .expect("first setup window")
        .window
        .clone();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });

    let surface = instance
        .create_surface(window.clone())
        .expect("failed to create surface");

    let adapter = application::async_std::task::block_on(instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        },
    ))
    .expect("failed to request adapter");

    let (device, queue) = application::async_std::task::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            required_features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
            ..Default::default()
        },
        None,
    ))
    .unwrap();

    let size = window.inner_size();

    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,

        width: size.width,
        height: size.height,

        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![wgpu::TextureFormat::Bgra8Unorm],
        desired_maximum_frame_latency: 0,
    };

    surface.configure(&device, &surface_config);

    let size = wgpu::Extent3d {
        // 2.
        width: surface_config.width,
        height: surface_config.height,
        depth_or_array_layers: 1,
    };

    let desc = wgpu::TextureDescriptor {
        label: Some("texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: super::DEPTH_TEXTURE_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    };

    let depth_buffer = device.create_texture(&desc);
    let depth_texture_view = depth_buffer.create_view(&wgpu::TextureViewDescriptor::default());

    Renderer {
        surface,
        device: device.into(),
        queue: Arc::new(queue),
        surface_config,
        adapter,
        depth_texture_view : Arc::new(depth_texture_view)
    }
}
