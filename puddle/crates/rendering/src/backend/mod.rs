mod buffers;
mod render_context;
mod setup;
mod pipeline;
pub use buffers::Buffer;
pub use pipeline::RenderPipelineDesc;

use bytemuck::checked::cast_slice;
pub use render_context::RenderContext;

use application::log::warn;
use legion::{system, IntoQuery};
use std::{sync::Arc, time::Instant};
use wgpu::{core::device::queue, util::DeviceExt};

pub struct WebGpu;

pub struct Renderer {
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
}



impl Renderer {
    pub(crate) fn new(app: &mut application::Application) -> Self {
        setup::init(app)
    }

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

    fn queue(&mut self) -> &mut wgpu::Queue {
        &mut self.queue
    }

    fn write_buffer(&mut self, buffer: Buffer, data: &[u8]) {
        self.queue.write_buffer(&buffer.buffer, 0, data);
    }

    pub fn create_buffer<T>(&self, usage: wgpu::BufferUsages, contents: &Vec<T>) -> Buffer
    where
        T: bytemuck::Pod,
    {
        use bytemuck::cast_slice;

        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: cast_slice(contents),
                usage,
            });

        Buffer {
            buffer,
            lengh: contents.len(),
        }
    }

}
