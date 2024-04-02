use super::*;

#[rustfmt::skip]
#[system]
pub fn setup(
    #[resource] renderer: &rendering::Renderer,
    #[resource] camera: &Camera,
    #[resource] async_loader : &mut asset_manager::AsyncModelQueue,
    #[state] spawned : &mut bool,
) {
    if *spawned {
        return;
    }

    *spawned = true;
    let pipeline = renderer.create_render_pipeline(&rendering::RenderPipelineDesc {
        shader: rendering::wgpu::include_wgsl!("../shader.wgsl"),
        bind_group_layouts: vec![&camera.bind_group_layout],
        ..rendering::RenderPipelineDesc::default()
    });

    let material = Arc::new(Material { pipeline, bind_group: None});

    let pipeline2 = renderer.create_render_pipeline(&rendering::RenderPipelineDesc {
        shader: rendering::wgpu::include_wgsl!("../shader.wgsl"),
        bind_group_layouts: vec![&camera.bind_group_layout],
        cull_mode : rendering::CullMode::Cw,
        ..rendering::RenderPipelineDesc::default()
    });

    let material2 = Arc::new(Material { pipeline: pipeline2, bind_group: None});

    async_loader.push( AsyncModelBuilder::new("Assets/Voxel.obj".to_string(), material.clone()).and_then(|entt, model, commands| {
        let reload = asset_manager::HotReloading::from_model_builder(model);
        commands.add_component(entt, reload);
    }));

    async_loader.push(AsyncModelBuilder::new("Assets/Clouds.obj".to_string(), material.clone()));
    async_loader.push(AsyncModelBuilder::new("Assets/Clouds2.obj".to_string(), material.clone()));
    async_loader.push(AsyncModelBuilder::new("Assets/Plant.obj".to_string(), material.clone()));
    async_loader.push(AsyncModelBuilder::new("Assets/SkyBox.obj".to_string(), material2.clone()));
}
