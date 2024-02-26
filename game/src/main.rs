mod camera;
mod cube;
mod pipeline;
mod chunk_loading;

use puddle::{
    application::Scheddules,
    rendering::{wgpu, CameraBindGroupLayout, Renderer, Vertex},
    texture,
};

use legion::{system, systems::CommandBuffer};
use std::{sync::Arc, time::Instant};

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

#[system]
fn load_chunk(
    commands: &mut CommandBuffer,
    #[resource] renderer: &mut Renderer,
    #[resource] pipeline: &mut ChunkPipeLine,
    #[resource] num: &mut CurrentChunk,
) {
    let (mut vertecies, indecies) = cube::get_data();
    let off = num.0 as f32 % 30.0;
    num.0 += 1;

    let row = (num.0 as f32 / 30.0).floor();

    for vert in vertecies.iter_mut() {
        vert.position[0] = -vert.position[0] + off * 2.0;
        vert.position[1] = -vert.position[1];
        vert.position[2] = -vert.position[2] + row * 2.0;
    }

    let mesh_data =
        puddle::rendering::RawMesh::new(renderer, vertecies, indecies, pipeline.0.clone());
    let chunk = commands.push(());
    commands.add_component(chunk, mesh_data);
}

struct ChunkPipeLine(Arc<wgpu::RenderPipeline>);

pub struct DeltaTime(f64);
struct LastUpdate(Instant);

struct CurrentChunk(u32);

#[system]
fn reset_deltatime(#[resource] lu: &mut LastUpdate, #[resource] dt: &mut DeltaTime) {
    let last_update = lu.0.elapsed().as_secs_f64();
    lu.0 = Instant::now();
    dt.0 = last_update;

    println!("rendering {} fps", (1.0 / dt.0).floor());
}

use std::thread;


fn main() {
    let mut app = puddle::application::Application::new();

    app.resources.insert(DeltaTime(0.0));
    app.resources.insert(LastUpdate(Instant::now()));
    app.resources.insert(CurrentChunk(0));

    app.resources.insert(Instant::now());

    app.add_plugin(puddle::window::WindowPlugin);
    app.add_plugin(puddle::rendering::RenderPlugin);

    app.scheddules
        .add_non_parralel(Scheddules::Startup, pipeline::load_pipeline);

    app.scheddules
        .add(Scheddules::Update, reset_deltatime_system());
    app.scheddules
        .add(Scheddules::Update, camera::camera_controller_system());
    app.scheddules
        .add(Scheddules::Update, chunk_loading::do_something_system(vec![]));

    app.scheddules.add(Scheddules::Update, load_chunk_system());

    //app.scheddules.add(Scheddules::Update, print_chunk_system());

    app.run();
}
