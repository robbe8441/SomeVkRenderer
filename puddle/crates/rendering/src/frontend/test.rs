use std::sync::Arc;

use crate::backend::Renderer;
use crate::frontend::RenderPass;

struct VertexBuffer(crate::backend::Buffer);
struct IndexBuffer(crate::backend::Buffer);
struct Pipeline(wgpu::RenderPipeline);

pub fn init(world: &mut legion::World, resources: &mut legion::Resources) {
    let mut renderer = match resources.get_mut::<Renderer>() {
        Some(r) => r,
        None => return,
    };

    let mut index_buffer = match resources.get_mut::<IndexBuffer>() {
        Some(r) => r,
        None => return,
    };

    let mut vertex_buffer = match resources.get_mut::<VertexBuffer>() {
        Some(r) => r,
        None => return,
    };

    let mut pipeline = match resources.get_mut::<Pipeline>() {
        Some(r) => r,
        None => return,
    };

    let mut contex = match renderer.create_render_context() {
        Some(r) => r,
        None => return,
    };

    contex.add_renderpass(RenderPass::ClearColor {
        color: [1.0, 1.0, 0.0, 0.0],
    });
    
    contex.add_renderpass(RenderPass::DrawIndexed {
        vertex_buffer: &vertex_buffer.0,
        index_buffer: &index_buffer.0,
        pipeline: &pipeline.0,
    });

    contex.flush(&mut renderer);
}



pub fn srtup(world: &mut legion::World, resources: &mut legion::Resources) {
    let mut renderer = match resources.get_mut::<Renderer>() {
        Some(r) => r,
        None => return,
    };

    use crate::types::Vertex;
    let vertex_buffer = renderer.create_buffer(wgpu::BufferUsages::VERTEX, &vec![
        Vertex {
            position : [1.0, 0.0, 0.0],
            uv_cords : [0.0, 0.0]
        },
        Vertex {
            position : [1.0, 1.0, 0.0],
            uv_cords : [0.0, 0.0]
        },
        Vertex {
            position : [0.0, 0.0, 0.0],
            uv_cords : [0.0, 0.0]
        },
    ]);
    let index_buffer = renderer.create_buffer(wgpu::BufferUsages::INDEX, &vec![0_u16,1,2]);
    let pipeline = renderer.create_render_pipeline(&crate::backend::RenderPipelineDesc::default());

    drop(renderer);

    resources.insert(VertexBuffer(vertex_buffer));
    resources.insert(IndexBuffer(index_buffer));
    resources.insert(Pipeline(pipeline));
}
