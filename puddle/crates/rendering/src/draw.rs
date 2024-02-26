use std::time::Instant;
use std::u16;

use crate::{Camera, CameraBindGroup, RenderCamera, Vertex};

use super::RawMesh;
use super::{render_context::RenderContext, Renderer};
use application::log::error;
use legion::{system, IntoQuery};
use wgpu::util::RenderEncoder;

fn clear_screen(context: &mut RenderContext) {
    context
        .command_encoder
        .begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &context.view.clone(),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        b: 0.01,
                        ..wgpu::Color::BLACK
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &context.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
}

pub fn draw(world: &mut legion::World, resources: &mut legion::Resources) {
    let mut renderer = match resources.get_mut::<Renderer>() {
        Some(r) => r,
        None => {
            return;
        }
    };
    let mut camera_bind_group = match resources.get_mut::<CameraBindGroup>() {
        Some(r) => r,
        None => {
            return;
        }
    };
    let mut camera_buffer = match resources.get_mut::<RenderCamera>() {
        Some(r) => r,
        None => {
            return;
        }
    };
    let mut camera = match resources.get_mut::<Camera>() {
        Some(r) => r,
        None => {
            return;
        }
    };

    {
        use std::sync::{Arc, Mutex};

        let event_poll = resources.get_mut::<Arc<Mutex<crate::RenderEvents>>>();

        let unpacked = match event_poll {
            Some(r) => r,
            None => {
                return;
            }
        };
        let mut locked = match unpacked.lock() {
            Ok(r) => r,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };

        if let Some(new_size) = &locked.resized {
            renderer.surface_config.width = new_size.0.width.max(10);
            renderer.surface_config.height = new_size.0.height.max(10);

            renderer
                .surface
                .configure(&renderer.device, &renderer.surface_config);

            locked.resized = None;
        }
    }

    let mut render_context = match RenderContext::new(
        &renderer.surface,
        &renderer.device,
        &renderer.surface_config,
    ) {
        Some(r) => r,
        None => {
            return;
        }
    };

    let view = render_context.view.clone();

    camera_buffer.uniform.update_view_proj(&camera);
    renderer.queue.write_buffer(
        &camera_buffer.buffer,
        0,
        bytemuck::cast_slice(&[camera_buffer.uniform]),
    );

    clear_screen(&mut render_context);

    let time = Instant::now();

    let mut num = 0;
    for mesh in <&RawMesh>::query().iter(world) {
        num += 1;

        let mut rpass =
            render_context
                .command_encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &render_context.depth_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

        rpass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        rpass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        rpass.set_pipeline(&mesh.pipeline);
        rpass.set_bind_group(0, &camera_bind_group.0, &[]);

        rpass.draw_indexed(
            0..mesh.index_buffer.size() as u32 / std::mem::size_of::<u16>() as u32,
            0,
            0..1,
        );
    }

    //println!("rendering : {} instances at : {} fps", num, 1.0 / time.elapsed().as_secs_f64());

    render_context.execute(&mut renderer.queue);
}
