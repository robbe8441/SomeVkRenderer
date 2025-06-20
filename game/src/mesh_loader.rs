use super::*;

#[system]
pub fn load_mesh(
    commands: &mut CommandBuffer,
    #[resource] renderer: &mut Renderer,
    #[resource] camera_bind_group: &puddle::rendering::CameraBindGroupLayout,
) {
    use noise::{NoiseFn, SuperSimplex};
    let simplex = SuperSimplex::new(0);
    let noise_size: f64 = 10.0;

    let size: u32 = 20;

    let mut result = Vec::with_capacity(size.pow(3) as usize);

    for i in 0..size.pow(3) {
        let x = i % size;
        let y = (i / size) % size;
        let z = (i / size.pow(2)) % size;

        let val = simplex.get([
            x as f64 / noise_size,
            y as f64 / noise_size,
            z as f64 / noise_size,
        ]);

        if val > 0.0 {
            result.push(255);
        } else {
            result.push(0);
        }
    }

    let tex_size = wgpu::Extent3d {
        width: size,
        height: size,
        depth_or_array_layers: size,
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
        &result,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(size),
            rows_per_image: Some(size),
        },
        tex_size,
    );

    let entries = vec![
        puddle::rendering::PuddleBindGroupEntry {
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Uint,
                view_dimension: wgpu::TextureViewDimension::D3,
                multisampled: false,
            },
            visibility: wgpu::ShaderStages::FRAGMENT,
            resource: wgpu::BindingResource::TextureView(&view),
        },
    ];

    let mut material = puddle::rendering::Material::new(
        renderer,
        entries,
        camera_bind_group,
        wgpu::include_wgsl!("./voxel_shader.wgsl"),
        true,
    );

    let data = cube::get_cube();
    material.add_mesh(data.0, data.1, puddle::rendering::ModelMatrix::default());

    let cube = commands.push(());
    commands.add_component(cube, material);
}
