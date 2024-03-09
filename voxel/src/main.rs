mod badapple;
mod camera;
mod chunk_gen;
mod skybox;
mod view;
use std::time::Instant;

use badapple::bad_apple_system;
use legion::system;
use puddle::application::{Application, Scheddules};

pub struct DeltaTime(f64);

#[system]
fn update_delta_time(#[state] last_frame: &mut Instant, #[resource] delta: &mut DeltaTime) {
    delta.0 = last_frame.elapsed().as_secs_f64();
    *last_frame = Instant::now();
}

pub struct PlaybackPuased(bool);

fn main() {
    chunk_gen::test();

    let mut app = Application::new();
    app.add_plugin(puddle::window::WindowPlugin);
    app.add_plugin(puddle::rendering::RenderPlugin);
    app.add_plugin(puddle::input::InputPlugin);

    app.resources.insert(DeltaTime(1.0));

    app.scheddules
        .add(Scheddules::Startup, skybox::add_skybox_system());

    app.scheddules
        .add(Scheddules::Startup, view::add_view_system());
    app.scheddules.add(
        Scheddules::Update,
        view::update_uniforms_system(Instant::now()),
    );
    app.scheddules
        .add(Scheddules::Update, camera::camera_controller_system());
    app.scheddules
        .add(Scheddules::Startup, camera::setup_cam_system());
    app.scheddules
        .add(Scheddules::Update, update_delta_time_system(Instant::now()));
    //app.scheddules.add(Scheddules::Update, view::load_chunk_system(vec![], 0));

    app.scheddules
        .add(Scheddules::Update, bad_apple_system(Instant::now()));

    app.resources.insert(PlaybackPuased(false));

    app.run();
}
