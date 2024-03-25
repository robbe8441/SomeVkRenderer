#![allow(unused, dead_code)]
mod test;
mod wgpu_backend;
pub use wgpu_backend::WebGpu;

pub trait RenderBackend {}
pub struct RenderPlugin<T: RenderBackend>(pub T);

pub(crate) trait RenderContext<T: Renderer> {
    //
    // create a new rendercontext
    fn begin(renderer: &mut T) -> Option<Box<Self>>
    where
        Self: Sized;

    // add a new command
    fn add_renderpass(&mut self, rpass: RenderPass);

    // execute and present the frame
    fn flush(self, renderer: &mut T);
}

pub(crate) trait Renderer {
    // crate a new buffer to send data to the gpu
    fn create_buffer(&mut self, ty: &BufferType) -> Buffer;

    // update data on the buffer
    fn write_buffer(&mut self, buffer: Buffer, data: &[u8]);

    fn queue(&mut self) -> &mut wgpu::Queue;

    /*
    fn begin_render(&mut self) -> Option<Box<dyn RenderContext<Self>>>
    where
        Self: Sized;
    */
}

use bytemuck::Zeroable;

#[derive(Clone)]
enum BufferType {
    Vertex(Vec<i32>),
    Index(Vec<i32>),
    Uniform(Vec<u8>),
}

pub(crate) struct Buffer {
    // TODO: use own buffer implementation to support other APIs
    buffer: wgpu::Buffer,
    ty: BufferType,
}

impl Buffer {
    fn update<T: bytemuck::Zeroable + bytemuck::Pod>(
        &mut self,
        renderer: &mut impl Renderer,
        data: &Vec<T>,
    ) {
        let data = bytemuck::cast_slice(&data);

        let size = self.buffer.size();

        // if its a diffrent lengh then create a new buffer, the old one is gonna be dropped automaticly
        if data.len() != size as usize {
            *self = renderer.create_buffer(&self.ty);
            return;
        }

        renderer.queue().write_buffer(&self.buffer, 0, data);
    }
}

pub(crate) enum RenderPass {
    ClearColor { color: [f64; 4] },

    DrawIndexed { vertex_buffer: Buffer },
}
