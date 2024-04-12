use legion::systems::CommandBuffer;

use super::*;

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct TestUniform {
    pub time: f32,
}

#[system(for_each)]
pub fn update_uniforms(
    uni: &mut TestUniform,
    buffer: &mut rendering::Buffer,
    #[state] time: &Instant,
    #[resource] renderer: &rendering::Renderer,
) {
    uni.time = (time.elapsed().as_secs_f32().sin() + 1.0) / 2.0;
    renderer.update_buffer(buffer, &[uni.clone()])
}



/// look at this ugly code
/// wth is that
/// what am i doing here ;-;
/// TODO: Fix my life
#[rustfmt::skip]
#[system]
pub fn setup(
    #[resource] renderer: &rendering::Renderer,
    #[resource] camera: &Camera,
    #[resource] async_loader : &mut asset_manager::AsyncModelQueue,
    #[state] spawned : &mut bool,
    commands : &mut CommandBuffer
) {
    if *spawned {
        return;
    }

    *spawned = true;

    let uniform_test = TestUniform { time: 69.0 };
    let uniforms = renderer.create_buffer(rendering::wgpu::BufferUsages::UNIFORM | rendering::wgpu::BufferUsages::COPY_DST, &[uniform_test]);

    let (test, test_layout) = rendering::BindGroupLayout {
        entries : vec![
            rendering::BindGroupLayoutEntry {
                visibility: rendering::wgpu::ShaderStages::FRAGMENT,
                ty: rendering::BindingType::Buffer(rendering::wgpu::BufferBindingType::Uniform),
                resource: uniforms.binding(),
            }
        ]
    }.build(renderer);


    let pipeline = renderer.create_render_pipeline(&rendering::RenderPipelineDesc {
        shader: rendering::wgpu::include_wgsl!("../shader.wgsl"),
        bind_group_layouts: vec![&camera.bind_group_layout, &test_layout],
        ..rendering::RenderPipelineDesc::default()
    });

    let material = Arc::new(Material { pipeline, bind_groups: vec![test]});

    async_loader.push( AsyncModelBuilder::new("Assets/Voxel.obj".to_string(), material.clone()).and_then(|entt, model, commands| {
        let reload = asset_manager::HotReloading::from_model_builder(model);
        commands.add_component(entt, reload);
    }));

    async_loader.push(AsyncModelBuilder::new("Assets/Clouds.obj".to_string(), material.clone()));
    async_loader.push(AsyncModelBuilder::new("Assets/Clouds2.obj".to_string(), material.clone()));

    let buffer = commands.push(());
    commands.add_component(buffer, uniform_test);
    commands.add_component(buffer, uniforms);
}
