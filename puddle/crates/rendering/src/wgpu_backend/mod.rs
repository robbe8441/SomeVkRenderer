mod plugin;
mod render_context;

use super::{RenderBackend, RenderPlugin};
use crate::{BufferType, RenderContext, RenderPass};

use application::log::warn;
use legion::{system, IntoQuery};
use wgpu::{core::device::queue, util::DeviceExt};

pub struct WebGpu;
impl RenderBackend for WebGpu {}

use std::{sync::Arc, time::Instant};

struct WgpuRenderer {
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
}

impl crate::Renderer for WgpuRenderer {
    /*
    fn begin_render(&mut self) -> Option<Box< dyn RenderContext<Self> >>
    where
        Self: Sized,
    {
        render_context::WgpuRenderContext::begin(self)
    }
    */

    fn create_buffer(&mut self, ty: &BufferType) -> crate::Buffer {
        use bytemuck::cast_slice;

        let (usage, contents) = match ty {
            BufferType::Index(v) => (wgpu::BufferUsages::INDEX, cast_slice(v)),
            BufferType::Vertex(v) => (wgpu::BufferUsages::VERTEX, cast_slice(v)),
            BufferType::Uniform(v) => (wgpu::BufferUsages::UNIFORM, cast_slice(v)),
        };

        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents,
                usage,
            });

        crate::Buffer {
            buffer,
            ty: ty.clone(),
        }
    }

    fn queue(&mut self) -> &mut wgpu::Queue {
        &mut self.queue
    }

    fn write_buffer(&mut self, buffer: crate::Buffer, data: &[u8]) {
        self.queue.write_buffer(&buffer.buffer, 0, data);
    }
}
