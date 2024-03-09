use legion::{system, systems::CommandBuffer};
use puddle::rendering::wgpu;
use puddle::rendering::{wgpu::include_wgsl, PuddleBindGroupEntry, Vertex};

#[system]
pub fn add_skybox(
    commands: &mut CommandBuffer,
    #[resource] renderer: &mut puddle::rendering::Renderer,
    #[resource] camera_bind_group: &mut puddle::rendering::CameraBindGroupLayout,
) {
    let image = image::load_from_memory(include_bytes!("./skybox.png")).unwrap();

    let texture =
        puddle::texture::Texture::from_image(&renderer.device, &renderer.queue, &image).unwrap();

    let entries = vec![
        PuddleBindGroupEntry {
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            resource: wgpu::BindingResource::TextureView(&texture.view),
            visibility: wgpu::ShaderStages::FRAGMENT,
        },
        PuddleBindGroupEntry {
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            resource: wgpu::BindingResource::Sampler(&texture.sampler),
            visibility: wgpu::ShaderStages::FRAGMENT,
        },
    ];

    let mut material = puddle::rendering::Material::new(
        renderer,
        entries,
        camera_bind_group,
        include_wgsl!("./sky_shader.wgsl"),
        false,
    );

    let (vertecies, indecies) = get_skybox();

    material.add_mesh(
        vertecies,
        indecies,
        puddle::rendering::ModelMatrix::default(),
    );

    let entry = commands.push(());
    commands.add_component(entry, material);
}

fn get_skybox() -> (Vec<Vertex>, Vec<u16>) {
    let vertices = vec![
        // Vorderseite
        Vertex {
            position: [-1.0, 1.0, -1.0],
            uv: [0.251, 0.334],
        }, // Bottom-left
        Vertex {
            position: [1.0, 1.0, -1.0],
            uv: [0.499, 0.334],
        }, // Bottom-right
        Vertex {
            position: [1.0, -1.0, -1.0],
            uv: [0.499, 0.665],
        }, // Top-right
        Vertex {
            position: [-1.0, -1.0, -1.0],
            uv: [0.251, 0.665],
        }, // Top-left
        // Rückseite
        Vertex {
            position: [1.0, 1.0, 1.0],
            uv: [0.751, 0.334],
        }, // Bottom-left
        Vertex {
            position: [-1.0, 1.0, 1.0],
            uv: [0.999, 0.334],
        }, // Bottom-right
        Vertex {
            position: [-1.0, -1.0, 1.0],
            uv: [0.999, 0.665],
        }, // Top-right
        Vertex {
            position: [1.0, -1.0, 1.0],
            uv: [0.751, 0.665],
        }, // Top-left
        // Links
        Vertex {
            position: [-1.0, 1.0, 1.0],
            uv: [0.001, 0.334],
        }, // Bottom-left
        Vertex {
            position: [-1.0, 1.0, -1.0],
            uv: [0.251, 0.334],
        }, // Bottom-right
        Vertex {
            position: [-1.0, -1.0, -1.0],
            uv: [0.251, 0.665],
        }, // Top-right
        Vertex {
            position: [-1.0, -1.0, 1.0],
            uv: [0.001, 0.665],
        }, // Top-left
        // Rechts
        Vertex {
            position: [1.0, 1.0, -1.0],
            uv: [0.501, 0.334],
        }, // Bottom-left
        Vertex {
            position: [1.0, 1.0, 1.0],
            uv: [0.751, 0.334],
        }, // Bottom-right
        Vertex {
            position: [1.0, -1.0, 1.0],
            uv: [0.751, 0.665],
        }, // Top-right
        Vertex {
            position: [1.0, -1.0, -1.0],
            uv: [0.501, 0.665],
        }, // Top-left
        // Oben
        Vertex {
            position: [-1.0, 1.0, 1.0],
            uv: [0.251, 0.001],
        }, // Bottom-left
        Vertex {
            position: [1.0, 1.0, 1.0],
            uv: [0.499, 0.001],
        }, // Bottom-right
        Vertex {
            position: [1.0, 1.0, -1.0],
            uv: [0.499, 0.334],
        }, // Top-right
        Vertex {
            position: [-1.0, 1.0, -1.0],
            uv: [0.251, 0.334],
        }, // Top-left
        // Unten
        Vertex {
            position: [-1.0, -1.0, -1.0],
            uv: [0.251, 0.666],
        }, // Bottom-left
        Vertex {
            position: [1.0, -1.0, -1.0],
            uv: [0.499, 0.666],
        }, // Bottom-right
        Vertex {
            position: [1.0, -1.0, 1.0],
            uv: [0.499, 0.999],
        }, // Top-right
        Vertex {
            position: [-1.0, -1.0, 1.0],
            uv: [0.251, 0.999],
        }, // Top-left
    ];

    let indices: Vec<u16> = vec![
        0, 1, 2, 2, 3, 0, // Vorderseite
        4, 5, 6, 6, 7, 4, // Rückseite
        8, 9, 10, 10, 11, 8, // Links
        12, 13, 14, 14, 15, 12, // Rechts
        16, 17, 18, 18, 19, 16, // Oben
        20, 21, 22, 22, 23, 20, // Unten
    ];

    (vertices, indices)
}
