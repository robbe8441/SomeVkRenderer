use crate::types::Vertex;
struct PuddleRenderipelineDesc {}

impl super::Renderer {
    pub fn create_render_pipeline(&mut self) -> wgpu::RenderPipeline {
        let pipeline_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("pipeline_layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let swapchain_capabilities = self.surface.get_capabilities(&self.adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let shader = self.device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),

                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },

                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: swapchain_format.into(),

                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::SrcAlpha,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,
                            },
                            alpha: wgpu::BlendComponent::OVER,
                        }),

                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),

                primitive: wgpu::PrimitiveState {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        render_pipeline
    }
}
