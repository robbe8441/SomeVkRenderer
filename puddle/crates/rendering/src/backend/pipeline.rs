use wgpu::include_wgsl;

use crate::types::Vertex;

pub enum CullMode {
    // counter clock wise
    Ccw,
    // clock wise
    Cw,
    // no culling
    None,
}



pub struct RenderPipelineDesc {
    shader: wgpu::ShaderModuleDescriptor<'static>,
    buffers: Vec<wgpu::VertexBufferLayout<'static>>,
    bind_group_layouts : Vec<&'static wgpu::BindGroupLayout>,
    allow_transparency: bool,
    cull_mode : CullMode,
}


impl Default for RenderPipelineDesc {
    fn default() -> Self {
        Self {
            shader: include_wgsl!("./shader.wgsl"),
            buffers: vec![Vertex::desc()],
            allow_transparency: false,
            cull_mode: CullMode::Ccw,
            bind_group_layouts: vec![],
        }
    }
}

impl super::Renderer {
    pub fn create_render_pipeline(&mut self, desc: &RenderPipelineDesc) -> wgpu::RenderPipeline {

        let swapchain_capabilities = self.surface.get_capabilities(&self.adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let shader = self
            .device
            .create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let blend = if desc.allow_transparency {
            Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent::OVER,
            })
        } else {
            None
        };


        use wgpu::{Face, FrontFace};
        let culling : (FrontFace, Option<Face>) = match desc.cull_mode {
            CullMode::Cw => (FrontFace::Cw, Some(Face::Back)),
            CullMode::Ccw => (FrontFace::Ccw, Some(Face::Back)),
            CullMode::None => (FrontFace::Ccw, None),
        };

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("pipeline_layout"),
                bind_group_layouts: &desc.bind_group_layouts,
                push_constant_ranges: &[],
            });

        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),

                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &desc.buffers,
                },

                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: swapchain_format.into(),
                        blend,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),

                primitive: wgpu::PrimitiveState {
                    front_face: culling.0,
                    cull_mode: culling.1,
                    ..Default::default()
                },

                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        render_pipeline
    }
}
