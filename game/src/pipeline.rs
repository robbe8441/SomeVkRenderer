use super::*;

pub fn load_pipeline(_world: &mut legion::World, resources: &mut legion::Resources) {
    let renderer = resources.get_mut::<Renderer>().unwrap();
    let camera_bind_group_layout = resources.get::<CameraBindGroupLayout>().unwrap().0.clone();

    let shader = renderer
        .device
        .create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

    let surface = &renderer.surface;
    let swapchain_capabilities = surface.get_capabilities(&renderer.adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let pipeline_layout = renderer
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline_layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

    let render_pipeline = renderer
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
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),

            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

    drop(renderer);

    resources.insert(ChunkPipeLine(Arc::new(render_pipeline)));
}
