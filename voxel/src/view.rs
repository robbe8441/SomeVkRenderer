use std::time::Instant;

use legion::{system, systems::CommandBuffer};
use puddle::rendering::{
    wgpu::{self, util::DeviceExt},
    CameraBindGroupLayout, Material, ModelMatrix, PuddleBindGroupEntry, Renderer, Vertex,
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    time: f32,
    width: f32,
    height: f32,
}

pub struct UniformBuffer(wgpu::Buffer);

#[system]
pub fn add_view(
    #[resource] renderer: &mut Renderer,
    #[resource] camera_bind_group_layout: &CameraBindGroupLayout,
    commands: &mut CommandBuffer,
) {
    let map_size: u32 = 100;
    let noise_res: f64 = 20.0;
    let mut data: Vec<u8> = Vec::with_capacity(map_size.pow(3) as usize);

    let noise = noise::SuperSimplex::new(0);
    use noise::NoiseFn;

    for x in 0..map_size {
        for y in 0..map_size {
            for z in 0..map_size {
                let val = noise.get([x as f64 / noise_res , y as f64 / noise_res, z as f64 / noise_res]);

                if val > 0.0 {
                    data.push(255);
                } else {
                    data.push(0);
                }
            }
        }
    }

    let tex_size = wgpu::Extent3d {
        width: map_size,
        height: map_size,
        depth_or_array_layers: map_size,
    };
    let texture = renderer.device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: tex_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D3,
        format: wgpu::TextureFormat::R8Uint,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    renderer.queue.write_texture(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &data,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(map_size),
            rows_per_image: Some(map_size),
        },
        tex_size,
    );

    let uniforms = Uniforms {
        time: 0.5,
        width: 0.0,
        height: 0.0,
    };

    let uniform_buffer = renderer
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

    let entries = vec![
        PuddleBindGroupEntry {
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            visibility: wgpu::ShaderStages::FRAGMENT,
            resource: wgpu::BindingResource::Buffer(uniform_buffer.as_entire_buffer_binding()),
        },
        PuddleBindGroupEntry {
            ty : wgpu::BindingType::Texture { 
                sample_type: wgpu::TextureSampleType::Uint,
                view_dimension: wgpu::TextureViewDimension::D3, 
                multisampled: false,
            },
            resource : wgpu::BindingResource::TextureView(&view),
            visibility : wgpu::ShaderStages::FRAGMENT,
        }


    ];

    let mut material = Material::new(
        renderer,
        entries,
        camera_bind_group_layout,
        wgpu::include_wgsl!("./shader.wgsl"),
        true,
    );

    let vertices = vec![
        Vertex {
            position: [0.0, 0.0, 0.0],
            uv: [-1.0, -1.0],
        },
        Vertex {
            position: [1.0, 0.0, 0.0],
            uv: [1.0, -1.0],
        },
        Vertex {
            position: [1.0, 1.0, 0.0],
            uv: [1.0, 1.0],
        },
        Vertex {
            position: [0.0, 1.0, 0.0],
            uv: [-1.0, 1.0],
        },
    ];

    let indecies = vec![0, 1, 2, 2, 3, 0];

    material.add_mesh(vertices, indecies, ModelMatrix::default());

    let entity = commands.push(());
    commands.add_component(entity, material);
    commands.add_component(entity, uniforms);
    commands.add_component(entity, UniformBuffer(uniform_buffer));
}

#[system(for_each)]
pub fn update_uniforms(
    uniform: &mut Uniforms,
    buffer: &UniformBuffer,
    #[state] time: &Instant,
    #[resource] renderer: &mut puddle::rendering::Renderer,
) {
    uniform.time = time.elapsed().as_secs_f32();

    uniform.width = renderer.surface_config.width as f32;
    uniform.height = renderer.surface_config.height as f32;

    renderer
        .queue
        .write_buffer(&buffer.0, 0, bytemuck::cast_slice(&[uniform.to_owned()]));
}
