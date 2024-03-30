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

pub struct RenderPipelineDesc<'a> {
    /// the shader used to draw the model
    /// right now this only supports one Shader
    pub shader: wgpu::ShaderModuleDescriptor<'a>,

    /// The formats of the vertex buffers
    /// default is just the Vertex layout, can be expanded to be used for instancing
    pub vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout<'a>>,

    /// the bind group layouts,
    /// used for textures, uniform buffers, and other buffers to be used in the shader
    pub bind_group_layouts: Vec<&'a wgpu::BindGroupLayout>,

    /// allows transparency,
    /// Warning : this disables the early Deph Test, what makes rendering slower,
    pub allow_transparency: bool,

    /// tells how to handle triangles,
    /// default is Ccw (counter clock wise)
    /// meaning the back face of the triangle isnt being rendered what makes rendering faster,
    /// in some cases you want the faces to be inverted
    /// or no culling at all
    pub cull_mode: CullMode,
}

impl Default for RenderPipelineDesc<'_> {
    fn default() -> Self {
        Self {
            // TODO : write better default shader, this one is trash
            shader: include_wgsl!("./shader.wgsl"),
            vertex_buffer_layouts: vec![Vertex::desc()],
            allow_transparency: false,
            cull_mode: CullMode::Ccw,
            bind_group_layouts: vec![],
        }
    }
}

impl super::Renderer {

    /// creates a new render_pipeline based on the description passed in
    pub fn create_render_pipeline(&self, desc: &RenderPipelineDesc) -> wgpu::RenderPipeline {
        let swapchain_capabilities = self.surface.get_capabilities(&self.adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let shader = self.device.create_shader_module(desc.shader.clone());

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

        let culling: (FrontFace, Option<Face>) = match desc.cull_mode {
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
                    buffers: &desc.vertex_buffer_layouts,
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

                // TODO : Add depth buffer
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        render_pipeline
    }
}
