mod bind_groups;
mod buffers;
mod pipeline;
mod render_context;
mod setup;

pub use buffers::Buffer;
pub use pipeline::{CullMode, RenderPipelineDesc};
pub use render_context::RenderContext;
pub use bind_groups::*;

use application::log::warn;
use legion::{system, IntoQuery};
use std::{sync::Arc, time::Instant};
use wgpu::{core::device::queue, util::DeviceExt};

use bytemuck::cast_slice;

/// the main renderer
pub struct Renderer {
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
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
        })
    }

    /// updates the data on the buffer
    /// replaces it with a new buffer if the lengh doesnt match
    #[inline(always)]
    pub fn update_buffer<T: bytemuck::Pod>(&self, buffer: &mut Buffer, data: &[T]) {
        // check if the currently loaded buffer is the same lengh
        // as wgpu doesnt allow resizing buffers,
        // so we need to create a new buffer

        if data.len() == buffer.lengh {
            self.queue.write_buffer(&buffer.buffer, 0, cast_slice(data));
        } else {
            *buffer = self.create_buffer(buffer.buffer.usage(), data);
        }
    }

    /// crates a new buffer used to store data on the gpu like veretcies
    /// usage tells wgpu what the buffer is used for
    /// "contents" is the data thats loaded on to the buffer / gpu
    #[inline(always)]
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
}
