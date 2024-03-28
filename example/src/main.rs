use legion::systems::CommandBuffer;
use puddle::asset_manager::import_model;
use puddle::rendering::RenderPlugin;
use puddle::*;

use legion::system;
use std::sync::Arc;
use std::time::Instant;

struct TestMaterial(Arc<puddle::asset_manager::Material>);

#[system(for_each)]
fn load_model(
    #[resource] renderer: &puddle::rendering::Renderer,
    #[state] count: &mut i32,
    #[state] last_update: &mut Instant,
    material: &TestMaterial,
    commands: &mut CommandBuffer,
) {
    *count += 1;

    let delta = last_update.elapsed().as_secs_f64();
    *last_update = Instant::now();

    println!("fps {}, at {}", 1.0 / delta, count);

    let model = import_model!("./TestModel.obj").build(renderer, material.0.clone());

    let entt = commands.push(());
    commands.add_component(entt, model);
    commands.add_component(entt, puddle::asset_manager::HotReloading::new("src/TestModel.obj".to_string()));
}

#[system]
fn setup(#[resource] renderer: &mut puddle::rendering::Renderer, commands: &mut CommandBuffer) {
    let pipeline = renderer.create_render_pipeline(&rendering::RenderPipelineDesc {
        shader: puddle::rendering::wgpu::include_wgsl!("./shader.wgsl"),
        ..Default::default()
    });

    let mat = puddle::asset_manager::Material { pipeline };

    let entt = commands.push(());

    commands.add_component(entt, TestMaterial(Arc::new(mat)));
}

fn main() {
    let mut app = Application::new();
    app.add_plugin(window::WindowPlugin);
    app.add_plugin(RenderPlugin);
    app.add_plugin(asset_manager::AssetManagerPlugin);

    app.scheddules
        .add(puddle::application::Scheddules::Startup, setup_system());

    app.scheddules.add(
        puddle::application::Scheddules::Update,
        load_model_system(0, Instant::now()),
    );

    app.run();
}
