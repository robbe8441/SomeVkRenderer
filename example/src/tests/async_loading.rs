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
///
/// TODO: make bind_group its own type
/// kinda something like :
///
/// ```rust
/// let mut bind_group = renderer.create_bind_group();
/// bind_group.add_uniforms([buffer]);  // problem -> no visibility setting
/// ```
///
///
/// combine the bind_group_layout and bind_group in to one type
///
/// little concept
/// ```rust
/// let mut bind_group = renderer
///     .create_bind_group()
///     .add_uniforms([buffer])
///     .add_camera([camera])
///     .build();
///
/// let pipeline = renderer.create_render_pipeline(&rendering::RenderPipelineDesc {
///     shader: include_wgsl!("../shader.wgsl"),
///     bind_groups: vec![&bind_group],
///     ..default()
/// });
///
/// let material = Arc::new(Material {
///     pipeline,
///     bind_groups: vec![bind_group],
/// });
///
///
/// ```
/// and make an custom option to pass it in like now, so its more customisable

#[system]
pub fn setup(
    #[resource] renderer: &rendering::Renderer,
    #[resource] camera: &Camera,
    #[resource] async_loader: &mut asset_manager::AsyncModelQueue,
    commands: &mut CommandBuffer,
) {

    let uniform_test = TestUniform { time: 69.0 };
    let uniforms = renderer.create_buffer(
        rendering::wgpu::BufferUsages::UNIFORM | rendering::wgpu::BufferUsages::COPY_DST,
        &[uniform_test],
    );

    let (test, test_layout) = rendering::BindGroupLayout {
        entries: vec![rendering::BindGroupLayoutEntry {
            visibility: rendering::wgpu::ShaderStages::FRAGMENT,
            ty: rendering::BindingType::Buffer(rendering::wgpu::BufferBindingType::Uniform),
            resource: uniforms.binding(),
        }],
    }
    .build(renderer);

    let pipeline = renderer.create_render_pipeline(&rendering::RenderPipelineDesc {
        shader: rendering::wgpu::include_wgsl!("../shader.wgsl"),
        bind_group_layouts: vec![&camera.bind_group_layout, &test_layout],
        ..rendering::RenderPipelineDesc::default()
    });

    let material = Arc::new(Material {
        pipeline,
        bind_groups: vec![test],
    });


    async_loader.push(AsyncModelBuilder::new(
        "Assets/Donut.obj".to_string(),
        material.clone(),
    ));

    let buffer = commands.push(());
    commands.add_component(buffer, uniform_test);
    commands.add_component(buffer, uniforms);
}
