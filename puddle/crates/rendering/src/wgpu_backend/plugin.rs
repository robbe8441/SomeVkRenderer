use super::*;


impl application::Plugin for RenderPlugin<WebGpu> {
    fn finish(&mut self, app: &mut application::Application) {
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

        app.resources.insert(WgpuRenderer {
            surface,
            device: device.into(),
            queue,
            surface_config,
            adapter,
        });
    }
}
