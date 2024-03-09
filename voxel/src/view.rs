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
    pub render_mode: i32,
}

pub struct UniformBuffer(wgpu::Buffer);
pub struct Chunktexture {
    pub texture: wgpu::Texture,
    size: wgpu::Extent3d,
}

pub const MAP_SIZE: u32 = 200;

fn gen_chunk() -> Vec<u8> {
    let mut data: Vec<u8> = Vec::with_capacity(MAP_SIZE.pow(3) as usize);

    let noise_res: f64 = 100.0;

    let seed = 0;

    let noise = noise::Worley::new(seed);
    let noise2 = noise::SuperSimplex::new(seed);
    let noise3 = noise::Value::new(seed);
    use noise::NoiseFn;

    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            for z in 0..MAP_SIZE {
                let pos = [
                    x as f64 / noise_res,
                    y as f64 / noise_res,
                    z as f64 / noise_res,
                ];
                let val = noise3.get(pos) + noise.get(pos) + noise2.get(pos);

                if val > 0.0 {
                    let val2 = noise2.get([x as f64 / 2.0, y as f64 / 2.0, z as f64 / 2.0]);
                    let val3 = noise3.get([x as f64 / 10.0, y as f64 / 10.0, z as f64 / 10.0]);
                    if val2 > 0.0 {
                        data.push(1);
                    } else if val3 > 0.0 {
                        data.push(2);
                    } else {
                        data.push(3);
                    }

                    continue;
                }
                data.push(0);
            }
        }
    }

    data
}

use std::thread;

#[system(for_each)]
pub fn load_chunk(
    texture: &mut Chunktexture,
    #[state] thread_pool: &mut Vec<thread::JoinHandle<Vec<u8>>>,
    #[state] state: &mut i32,
    #[resource] renderer: &mut Renderer,
) {
    if *state == 0 {
        let handle = thread::spawn(gen_chunk);
        thread_pool.push(handle);
        *state = 1;
    }

    let handle = thread_pool.first();

    if *state == 1 && handle.is_some() {
        if handle.unwrap().is_finished() {
            let handle = thread_pool.remove(0);
            let data = handle.join().unwrap();

            renderer.queue.write_texture(
                wgpu::ImageCopyTexture {
                    aspect: wgpu::TextureAspect::All,
                    texture: &texture.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                &data,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(MAP_SIZE),
                    rows_per_image: Some(MAP_SIZE),
                },
                texture.size,
            );

            *state = 2;
        }
    }
}

#[system]
pub fn add_view(
    #[resource] renderer: &mut Renderer,
    #[resource] camera_bind_group_layout: &CameraBindGroupLayout,
    commands: &mut CommandBuffer,
) {
    println!("{}", MAP_SIZE.pow(3) * std::mem::size_of::<u8>() as u32);

    let tex_size = wgpu::Extent3d {
        width: 962,
        height: 720,
        depth_or_array_layers: 1,
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

    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(255);
    colors.push([1.0, 1.0, 1.0, 1.0]);

    colors.push([0.2, 0.2, 0.2, 1.0]);
    colors.push([0.25, 0.25, 0.25, 1.0]);
    colors.push([0.16, 0.16, 0.16, 1.0]);

    let color_buffer = renderer
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("colro buffer"),
            contents: bytemuck::cast_slice(&colors),
            usage: wgpu::BufferUsages::STORAGE,
        });

    let uniforms = Uniforms {
        time: 0.5,
        width: 0.0,
        height: 0.0,
        render_mode: 100,
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
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Uint,
                view_dimension: wgpu::TextureViewDimension::D3,
                multisampled: false,
            },
            resource: wgpu::BindingResource::TextureView(&view),
            visibility: wgpu::ShaderStages::FRAGMENT,
        },
        PuddleBindGroupEntry {
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            visibility: wgpu::ShaderStages::FRAGMENT,
            resource: wgpu::BindingResource::Buffer(color_buffer.as_entire_buffer_binding()),
        },
    ];

    let mut material = Material::new(
        renderer,
        entries,
        camera_bind_group_layout,
        wgpu::include_wgsl!("./shader.wgsl"),
        false,
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
    commands.add_component(
        entity,
        Chunktexture {
            texture,
            size: tex_size,
        },
    );
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
