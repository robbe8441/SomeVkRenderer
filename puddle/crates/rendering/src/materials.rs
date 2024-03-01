use std::sync::Arc;
use super::instaincing::*;
use wgpu::util::DeviceExt;

pub struct Material {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_groups: Arc<wgpu::BindGroup>,

    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,

    pub(crate) vertecies : Vec<Vertex>,
    pub(crate) indecies : Vec<u16>,

    pub instances : Vec<ModelMatrix>,

    device : Arc<wgpu::Device>,
}

pub struct PuddleBindGroupEntry<'a> {
    pub ty: wgpu::BindingType,
    pub visibility: wgpu::ShaderStages,
    pub resource: wgpu::BindingResource<'a>,
}

impl Material {
    pub fn new(
        renderer: &mut crate::Renderer,
        entries: Vec<PuddleBindGroupEntry>,
        camera_bind_group: &CameraBindGroupLayout,
        shader: wgpu::ShaderModuleDescriptor,
    ) -> Self {
        let vertex_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vertex buffer"),
                usage: wgpu::BufferUsages::VERTEX,
                contents: &[],
            });

        let index_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vertex buffer"),
                usage: wgpu::BufferUsages::INDEX,
                contents: &[],
            });

        let instance_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vertex buffer"),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::VERTEX,
                contents: &[],
            });


        let layout_entries: Vec<wgpu::BindGroupLayoutEntry> = entries
            .iter()
            .enumerate()
            .map(|(i, x)| wgpu::BindGroupLayoutEntry {
                binding: i as u32,
                visibility: x.visibility,
                ty: x.ty,
                count: None, // TODO what is this   looks interesting
            })
            .collect();

        let entries: Vec<wgpu::BindGroupEntry> = entries
            .into_iter()
            .enumerate()
            .map(|(i, x)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: x.resource,
            })
            .collect();

        let bindgroup_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &layout_entries,
                });

        let bind_groups = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("bind group"),
                layout: &bindgroup_layout,
                entries: &entries,
            });

        let pipeline = load_pipeline(renderer, camera_bind_group, shader, &bindgroup_layout);

        Self {
            bind_groups: bind_groups.into(),
            vertex_buffer,
            index_buffer,
            instance_buffer,
            pipeline,
            device : renderer.device.clone(),
            vertecies : vec![],
            indecies : vec![],
            instances : vec![],
        }
    }

    pub fn add_mesh(&mut self, mut vertecies : Vec<Vertex>, mut indecies : Vec<u16>, position : ModelMatrix) {
        self.vertecies.extend(vertecies.drain(..));
        self.indecies.extend(indecies.drain(..));
        self.instances.push(position);

        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertecies),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.indecies),
            usage: wgpu::BufferUsages::INDEX,
        });

        let raw_data : Vec<_> = self.instances.iter().map(|x| x.to_raw()).collect();

        let instance_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&raw_data),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::VERTEX,
        });


        self.vertex_buffer = vertex_buffer;
        self.instance_buffer = instance_buffer;
        self.index_buffer = index_buffer;
    }
}










use super::*;

pub fn load_pipeline(
    renderer: &mut crate::Renderer,
    camera_bind_group: &CameraBindGroupLayout,
    shader: wgpu::ShaderModuleDescriptor,
    bind_group : &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = renderer.device.create_shader_module(shader);

    let surface = &renderer.surface;
    let swapchain_capabilities = surface.get_capabilities(&renderer.adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let pipeline_layout = renderer
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline_layout"),
            bind_group_layouts: &[&camera_bind_group.0, &bind_group],
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
                buffers: &[Vertex::desc(), InstanceRaw::desc()],
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

    render_pipeline
}
