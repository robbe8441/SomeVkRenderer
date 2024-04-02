mod bind_groups;
mod buffers;
mod pipeline;
mod render_context;
mod setup;

pub use bind_groups::*;
pub use buffers::Buffer;
pub use pipeline::{CullMode, RenderPipelineDesc};
pub use render_context::RenderContext;

use application::log::warn;
use legion::{system, IntoQuery};
use std::{
    marker::{Send, Sync},
    sync::Arc,
    time::Instant,
};
use wgpu::{core::device::queue, util::DeviceExt};

use bytemuck::cast_slice;


pub const DEPTH_TEXTURE_FORMAT : wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

/// the main renderer
pub struct Renderer {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub surface: wgpu::Surface<'static>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
    pub depth_texture_view: Arc<wgpu::TextureView>,
}

impl Renderer {
    /// creates a new wgpu renderer and stores it as resource inside the app
    #[inline]
    pub(crate) fn new(app: &mut application::Application) -> Self {
        setup::init(app)
    }

    /// crates a new render_context
    /// a single use container to draw a frame
    /// example
    /// ```
    ///  let render_context = renderer.create_render_context().unwrap();
    ///
    ///  render_context.add_renderpass(RenderPass::ClearColor { color : [1.0, 0.0, 0.0, 0.0] });
    ///     
    ///  render_context.flush(renderer);
    ///
    ///````
    pub fn create_render_context(&mut self) -> Option<RenderContext> {
        let command_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Wgpu Command Encoder"),
            });

        let frame = match self.surface.get_current_texture() {
            Ok(r) => r,
            Err(e) => {
                warn!("dropped frame {}", e);
                return None;
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Some(RenderContext {
            view: Arc::new(view),
            frame,
            command_encoder,
            depth_buffer_view: self.depth_texture_view.clone(),
        })
    }

    /// updates the data on the buffer
    /// replaces it with a new buffer if the lengh doesnt match
    pub fn update_buffer<T: bytemuck::Pod>(
        &self,
        buffer: &mut Buffer,
        data: &[T],
    ) {
        // check if the currently loaded buffer is the same lengh
        // as wgpu doesnt allow resizing buffers,
        // so we need to create a new buffer

        let queue = self.queue.clone();
        let buffer_clone = buffer.clone();
        let data = data.clone();

        if data.len() == buffer.lengh {
            queue.write_buffer(&buffer_clone.buffer, 0, cast_slice(&data));
        } else {
            *buffer = self.create_buffer(buffer.buffer.usage(), &data);
        }
    }

    /// crates a new buffer used to store data on the gpu like veretcies
    /// usage tells wgpu what the buffer is used for
    /// "contents" is the data thats loaded on to the buffer / gpu
    pub fn create_buffer<T: bytemuck::Pod>(
        &self,
        usage: wgpu::BufferUsages,
        contents: &[T],
    ) -> Buffer {
        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: cast_slice(contents),
                usage,
            });

        Buffer {
            buffer: Arc::new(buffer),
            lengh: contents.len(),
        }
    }

    pub fn create_texture(&self) -> wgpu::Texture {
        let size = wgpu::Extent3d {
            // 2.
            width: self.surface_config.width,
            height: self.surface_config.height,
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some("texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };

        self.device.create_texture(&desc)
    }
}
