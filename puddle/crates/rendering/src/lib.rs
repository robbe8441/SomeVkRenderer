#![allow(unused, dead_code)]
mod draw;
mod instaincing;
mod camera;
mod materials;
mod meshes;
mod render_context;

pub use camera::{Camera, CameraUniform};
pub use instaincing::*;
pub use materials::*;
pub use meshes::*;
pub use wgpu;
pub struct RenderPlugin;
pub use draw::CustomDepthBuffer;

use application::Plugin;
use std::sync::Arc;

pub struct Renderer {
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
}

pub struct CameraBindGroup(pub wgpu::BindGroup);
pub struct CameraBindGroupLayout(pub Arc<wgpu::BindGroupLayout>);

struct RenderCamera {
    pub buffer: wgpu::Buffer,
    pub uniform: CameraUniform,
}


use legion::{system, IntoQuery};
use wgpu::{core::device::queue, util::DeviceExt};

impl Plugin for RenderPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        let window = app
            .resources
            .get::<window::PuddleWindow>()
            .unwrap()
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

        let (device, queue) = application::async_std::task::block_on(
            adapter.request_device(&wgpu::DeviceDescriptor {
                required_features : wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                ..Default::default()
            }, None),
        )
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

        use application::Scheddules;
        app.scheddules
            .add_non_parralel(Scheddules::Update, draw::draw);

        let render_events = Arc::new(std::sync::Mutex::new(RenderEvents { resized: None }));

        /// setup camera
        let mut cam = Camera::default(
            surface_config.width as f32 / surface_config.height as f32,
        );
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&cam);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let cam_buffers = RenderCamera {
            buffer: camera_buffer,
            uniform: camera_uniform,
        };

        app.resources.insert(CameraBindGroup(camera_bind_group));
        app.resources.insert(CameraBindGroupLayout(Arc::new(camera_bind_group_layout)));
        app.resources.insert(cam_buffers);
        app.resources.insert(cam);
        app.resources.insert(render_events);
        app.resources.insert(Renderer {
            surface,
            device : device.into(),
            queue,
            surface_config,
            adapter,
        });
    }
}

