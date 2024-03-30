mod functions;
use application::log::warn;

use super::Renderer;
use crate::frontend::RenderPass;
use std::sync::Arc;

pub struct RenderContext {
    pub view: Arc<wgpu::TextureView>,
    pub frame: wgpu::SurfaceTexture,
    pub command_encoder: wgpu::CommandEncoder,
}

impl RenderContext {
    // add a new command
    pub fn add_renderpass(&mut self, rpass: RenderPass) {
        match rpass {
            RenderPass::ClearColor { color } => self.clear_color(color),

            RenderPass::DrawIndexed {
                vertex_buffer,
                index_buffer,
                pipeline,
                bind_groups,
            } => self.draw_indexed(vertex_buffer, index_buffer, pipeline, bind_groups),
        }
    }

    // execute and present the frame
    pub fn flush(self, renderer: &mut Renderer) {
        let buffer = self.command_encoder.finish();
        renderer.queue.submit([buffer]);
        self.frame.present();
    }
}
