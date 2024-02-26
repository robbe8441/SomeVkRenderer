use super::*;

#[derive(Debug)]
pub struct RawMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub pipeline: Arc<wgpu::RenderPipeline>,
}

impl RawMesh {
    pub fn new(
        renderer: &mut Renderer,
        vertecies: Vec<Vertex>,
        indecies: Vec<u16>,
        pipeline: Arc<wgpu::RenderPipeline>,
    ) -> Self {
        let vertex_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vertex buffer"),
                contents: bytemuck::cast_slice(&vertecies),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("index buffer"),
                contents: bytemuck::cast_slice(&indecies),
                usage: wgpu::BufferUsages::INDEX,
            });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2, // NEW!
                },
            ],
        }
    }
}

