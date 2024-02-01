use std::{sync::Arc, time::Instant};

use super::*;
mod renderpass;

pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_desc: wgpu::SurfaceConfiguration,

    pub surface: wgpu::Surface<'static>,
    pub adapter: wgpu::Adapter,

    pub imgui_render: Option<puddle_imgui::imgui_renderer::PuddleImGuiRenderer>,
}

impl Renderer {
    pub fn new(
        tokio_runtime: &tokio::runtime::Runtime,
        window: Arc<winit::window::Window>,
    ) -> Renderer {

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

        info!("took {}s to load renderer", start_time.elapsed().as_secs_f64() );

        Renderer {
            surface,
            adapter,
            device,
            queue,
            surface_desc,
            imgui_render: None,
        }
    }
}

impl Renderer {
    pub fn draw(&mut self, window: Arc<winit::window::Window>) {
        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(e) => {
                warn!("dropped frame: {e:?}");
                return;
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder: wgpu::CommandEncoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        if self.imgui_render.is_some() {
            self.imgui_render.as_mut().unwrap().draw_puddle_imgui(
                &self.device,
                &self.queue,
                window,
                &mut encoder,
                &view,
            );
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
