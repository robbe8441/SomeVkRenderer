use legion::system;
use legion::systems::CommandBuffer;
use puddle::application::Scheddules;
use puddle::*;
use std::path::Path;

use std::sync::Arc;
use std::time::{Duration, Instant};

#[rustfmt::skip]
#[system]
fn setup(
    #[resource] renderer: &rendering::Renderer,
    #[resource] camera: &asset_manager::camera::Camera,
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

    let material = Arc::new(puddle::asset_manager::Material { pipeline });

    let loader = asset_manager::load_model(&Path::new("Assets/TestModel.obj"));
    let reload = loader.get_hot_reload().unwrap();

    let model = loader.build(renderer, material.clone());

    let entt = commands.push(());
    commands.add_component(entt, model);
    commands.add_component(entt, reload);
}


#[system]
fn update_cam(
    #[resource] renderer: &rendering::Renderer,
    #[resource] camera: &mut puddle::asset_manager::camera::Camera,
    #[state] time : &Instant
    ) {

    const R : f32 = 1.5;
    let t = time.elapsed().as_secs_f32();
    let x = t.cos() * R;
    let y = t.sin() * R;

    camera.data.eye.x = x;
    camera.data.eye.z = y;

    camera.camera_uniform.view_proj = camera.data.build_view_projection_matrix().into();
    renderer.update_buffer(&mut camera.uniform_buffer, &[camera.camera_uniform]);
}



fn main() {
    let mut app = puddle::application::Application::new();

    app.add_plugin(window::WindowPlugin);
    app.add_plugin(rendering::RenderPlugin);
    app.add_plugin(asset_manager::AssetManagerPlugin);

    app.scheddules.add(Scheddules::Update, setup_system(false));
    app.scheddules.add(Scheddules::Update, update_cam_system(Instant::now()));

    app.run();
}
