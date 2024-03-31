use legion::system;
use legion::systems::CommandBuffer;
use puddle::*;
use puddle::{
    application::Scheddules,
    rendering::utils::{Camera, Material},
    asset_manager::load_model,
};
use std::path::Path;

use std::sync::Arc;
use std::time::Instant;

#[rustfmt::skip]
#[system]
fn setup(
    #[resource] renderer: &rendering::Renderer,
    #[resource] camera: &Camera,
    #[state] spawned : &mut bool,
    commands: &mut CommandBuffer,
) {
    if *spawned {
        return;
    }

    *spawned = true;
    let pipeline = renderer.create_render_pipeline(&rendering::RenderPipelineDesc {
        shader: rendering::wgpu::include_wgsl!("./shader.wgsl"),
        cull_mode: rendering::CullMode::Ccw,
        bind_group_layouts: vec![&camera.bind_group_layout],
        
        ..Default::default()
    });

    let material = Arc::new(Material { pipeline, bind_group: None});

    let loader = load_model(&Path::new("Assets/Voxel.obj"));
    let reload = puddle::asset_manager::HotReloading::from_model_builder(&loader);

    let model = loader.build(renderer, material.clone());

    let entt = commands.push(());
    commands.add_component(entt, model);
    commands.add_component(entt, reload);
}

#[system]
fn update_cam(
    #[resource] renderer: &rendering::Renderer,
    #[resource] camera: &mut Camera,
    #[state] time: &Instant,
) {
    const R: f32 = 7.0;
    let t = time.elapsed().as_secs_f32() / 4.0;
    let x = (t * 2.0).cos() * R;
    let z = (t * 2.0).sin() * R;

    camera.data.eye = [x, 2.0, z].into();

    camera.camera_uniform.view_proj = camera.data.build_view_projection_matrix().into();
    renderer.update_buffer(&mut camera.uniform_buffer, &[camera.camera_uniform].into());
}


fn main() {
    let mut app = puddle::application::Application::new();

    app.add_plugin(window::WindowPlugin);
    app.add_plugin(rendering::RenderPlugin);
    app.add_plugin(asset_manager::AssetManagerPlugin);

    app.scheddules.add(Scheddules::Update, setup_system(false));
    app.scheddules
        .add(Scheddules::Update, update_cam_system(Instant::now()));

    app.run();
}
