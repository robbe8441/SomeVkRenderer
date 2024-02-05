mod renderpass;

use super::*;
use std::{sync::Arc, time::Instant};
pub use renderpass::RenderContext;

pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_desc: wgpu::SurfaceConfiguration,

    pub surface: wgpu::Surface<'static>,
    pub adapter: wgpu::Adapter,
}


impl Renderer {
    pub fn new(window : Arc<winit::window::Window>, tokio_runtime : &tokio::runtime::Runtime) -> Self {
        info!("setting up renderer");
        let start_time = Instant::now();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance
            .create_surface(window.clone())
            .expect("failed to create surface");

        let adapter = tokio_runtime
            .block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))
            .expect("failed to request adapter");

        let (device, queue) = tokio_runtime
            .block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None))
            .unwrap();

        let size = window.inner_size();

        let surface_desc = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,

            width: size.width,
            height: size.height,

            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![wgpu::TextureFormat::Bgra8Unorm],
            desired_maximum_frame_latency: 0,
        };

        surface.configure(&device, &surface_desc);

        let renderer = Renderer {
            surface, surface_desc, device, adapter, queue,
        };

        info!("took {}s to load renderer", start_time.elapsed().as_secs_f64());

        renderer
    }
}
