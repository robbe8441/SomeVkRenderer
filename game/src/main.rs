mod camera;
mod cube;
mod mesh_loader;

use puddle::{
    application::Scheddules,
    rendering::{wgpu, Renderer},
};

use legion::{system, systems::CommandBuffer};
use rand::Rng;
use std::time::Instant;

#[system]
fn update_cam(
    #[resource] camera: &mut puddle::rendering::Camera,
    #[resource] start_time: &Instant,
) {
    let t = start_time.elapsed().as_secs_f32();
    let z = t.sin();
    let x = t.cos();
    camera.eye.z = z * 2.0;
    camera.eye.x = x * 2.0;
}

pub struct DeltaTime(f64);
struct LastUpdate(Instant);

#[system(for_each)]
fn reset_deltatime(
    #[resource] lu: &mut LastUpdate,
    #[state] val: &mut u64,
    material: &puddle::rendering::Material,
) {
    let last_update = lu.0.elapsed().as_secs_f64();
    lu.0 = Instant::now();

    *val += 1;
    if *val % 100 == 0 {
        println!(
            "{} fps at {} instances",
            1.0 / last_update,
            material.instances.len()
        );
    }
}

#[system(for_each)]
fn load_mesh(
    material: &mut puddle::rendering::Material,
) {
    let data = cube::get_cube();

    let mut rng = rand::thread_rng();
    let offset_x = rng.gen_range(-20.0..20.0);
    let offset_y = rng.gen_range(-20.0..20.0);
    let offset_z = rng.gen_range(-20.0..20.0);

    let mut pos = puddle::rendering::ModelMatrix::default();
    pos.position.x += offset_x;
    pos.position.y += offset_y;
    pos.position.z += offset_z;

    material.add_mesh(data.0, data.1, pos);
}

fn main() {
    let mut app = puddle::application::Application::new();

    app.resources.insert(DeltaTime(0.0));
    app.resources.insert(LastUpdate(Instant::now()));

    app.resources.insert(Instant::now());

    app.add_plugin(puddle::window::WindowPlugin);
    app.add_plugin(puddle::rendering::RenderPlugin);
    app.add_plugin(puddle::input::InputPlugin);

    // add systems

    app.resources.insert(DeltaTime(1.0));

    app.scheddules
        .add(Scheddules::Update, reset_deltatime_system(0));
    app.scheddules.add(Scheddules::Update, camera::camera_controller_system());
    app.scheddules
        .add(Scheddules::Startup, mesh_loader::load_mesh_system());
    app.scheddules.add(Scheddules::Update, load_mesh_system());

    app.run();
}
