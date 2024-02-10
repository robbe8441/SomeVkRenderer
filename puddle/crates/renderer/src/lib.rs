mod context;

use logger::{error, info, warn};
use std::fs::rename;
use std::slice::from_raw_parts;
use std::sync::Arc;
use std::time::Instant;
use window::PuddleWindow;

pub use context::RenderContext;

pub struct PuddleRenderer<'a> {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface<'a>,
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub render_passes: Vec<Arc<dyn Fn(&mut PuddleRenderer, &mut RenderContext)>>,
    pub view: Option<wgpu::TextureView>,
}

pub struct DefaultRenderer;

use legion::*;

fn clear_screen(renderer: &mut PuddleRenderer, context: &mut RenderContext) {
    context.begin_render_pass(wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &renderer.view.as_ref().unwrap(),
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
    });
}

fn draw(world: &mut legion::World, resources: &mut legion::Resources) {
    let mut renderer = match resources.get_mut::<PuddleRenderer>() {
        Some(r) => r,
        None => {
            error!("fariled to get renderer");
            return;
        }
    };

    let frame = match renderer.surface.get_current_texture() {
        Ok(frame) => frame,
        Err(e) => {
            warn!("dropped frame : {e}");
            return;
        }
    };

    let mut render_context = context::RenderContext::new(renderer.device.clone());

    renderer.view = Some(
        frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default()),
    );

    for rpass in <Vec<
        Arc<dyn for<'a, 'b, 'c, 'd> Fn(&'a mut PuddleRenderer<'b>, &'c mut RenderContext<'d>)>,
    > as Clone>::clone(&renderer.render_passes)
    .into_iter()
    {
        rpass(&mut renderer, &mut render_context);
    }

    renderer.queue.submit(render_context.finish().into_iter());
    frame.present();
}


impl application::Plugin for DefaultRenderer {
    fn second_build_stage(&mut self, app: &mut application::Application) {
        logger::info!("setting up renderer");

        let window = app
            .resources
            .get_mut::<window::PuddleWindow>()
            .expect("first setup window")
            .clone();
        let tokio_runtime = app
            .resources
            .get_mut::<Arc<application::tokio::runtime::Runtime>>()
            .expect("first setup app")
            .clone();

        let start_time = Instant::now();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance
            .create_surface(window.get_cloned())
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

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,

            width: size.width,
            height: size.height,

            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![wgpu::TextureFormat::Bgra8Unorm],
            desired_maximum_frame_latency: 0,
        };

        surface.configure(&device, &surface_config);

        let mut renderer = PuddleRenderer {
            surface,
            device: Arc::new(device),
            instance,
            queue,
            surface_config,
            render_passes: vec![],
            view: None,
        };

        renderer.add_renderpass(clear_screen);

        app.resources.insert(renderer);
        info!("added renderer");
        app.schedule.on_update.add_thread_local_fn(draw);

        info!(
            "took {}s to load renderer",
            start_time.elapsed().as_secs_f64()
        );
    }
}

impl PuddleRenderer<'_> {
    pub fn add_renderpass(
        &mut self,
        rpass: impl Fn(&mut PuddleRenderer, &mut RenderContext) + 'static,
    ) {
        self.render_passes.push(Arc::new(rpass));
    }
}
