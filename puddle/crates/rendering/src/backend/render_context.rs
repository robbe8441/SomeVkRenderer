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
        let color_attachments = if let RenderPass::ClearColor { color } = rpass {
            let color = wgpu::Color {
                r: color[0],
                g: color[1],
                b: color[2],
                a: color[3],
            };

            Some(wgpu::RenderPassColorAttachment {
                view: &self.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: wgpu::StoreOp::Store,
                },
            })
        } else {
            None
        };

        self.command_encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[color_attachments],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
    }

    // execute and present the frame
    pub fn flush(self, renderer: &mut Renderer) {
        let buffer = self.command_encoder.finish();
        renderer.queue.submit([buffer]);
        self.frame.present();
    }
}
