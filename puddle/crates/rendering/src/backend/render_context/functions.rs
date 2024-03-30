use wgpu::util::RenderEncoder;

use crate::backend::Buffer;

use super::RenderContext;
use crate::frontend::RenderPass;
use std::sync::Arc;

impl RenderContext {
    pub fn clear_color(&mut self, color: [f64; 4]) {
        let color = wgpu::Color {
            r: color[0],
            g: color[1],
            b: color[2],
            a: color[3],
        };

        let color_attachment = Some(wgpu::RenderPassColorAttachment {
            view: &self.view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(color),
                store: wgpu::StoreOp::Store,
            },
        });

        self.command_encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Color"),
                color_attachments: &[color_attachment],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
    }

    pub fn draw_indexed(
        &mut self,
        vertex_buffer: &Buffer,
        index_buffer: &Buffer,
        pipeline: &wgpu::RenderPipeline,
        bind_groups: &Vec<&wgpu::BindGroup>,
    ) {
        let color_attachments = Some(wgpu::RenderPassColorAttachment {
            view: &self.view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
        });

        let mut rpass = self
            .command_encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Draw Indexed"),
                color_attachments: &[color_attachments],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

        for (i, bind_group) in bind_groups.iter().enumerate() {
            rpass.set_bind_group(i as u32, bind_group, &[]);
        }
        rpass.set_pipeline(&pipeline);
        rpass.set_vertex_buffer(0, vertex_buffer.buffer.slice(..));
        rpass.set_index_buffer(index_buffer.buffer.slice(..), wgpu::IndexFormat::Uint32);
        rpass.draw_indexed(0..index_buffer.lengh as u32, 0, 0..1);

    }
}
