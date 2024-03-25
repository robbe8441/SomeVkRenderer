use application::log::warn;

use super::WgpuRenderer;
use crate::{RenderPass, WebGpu};
use std::sync::Arc;

pub(crate) struct WgpuRenderContext {
    pub view: Arc<wgpu::TextureView>,
    pub frame: wgpu::SurfaceTexture,
    pub command_encoder: wgpu::CommandEncoder,
}

impl crate::RenderContext<WgpuRenderer> for WgpuRenderContext {
    // create a new rendercontext
    fn begin(renderer: &mut WgpuRenderer) -> Option<Box<Self>> {
        let command_encoder =
            renderer
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Wgpu Command Encoder"),
                });

        let frame = match renderer.surface.get_current_texture() {
            Ok(r) => r,
            Err(e) => {
                warn!("dropped frame {}", e);
                return None;
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Some(Box::new(WgpuRenderContext {
            view: Arc::new(view),
            frame,
            command_encoder,
        }))
    }

    // add a new command
    fn add_renderpass(&mut self, rpass: RenderPass) {
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
    fn flush(self, renderer: &mut WgpuRenderer) {
        let buffer = self.command_encoder.finish();
        renderer.queue.submit([buffer]);
        self.frame.present();
    }
}
