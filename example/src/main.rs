#![allow(unused, dead_code)]

mod tests;
use std::time::Instant;

use application::Schedules;
use puddle::*;


#[legion::system]
fn create_uniform(#[resource] renderer: &mut rendering::Renderer) {
}

fn main() {
    let mut app = puddle::application::Application::new();

    app.add_plugin(window::WindowPlugin);
    app.add_plugin(rendering::RenderPlugin);
    app.add_plugin(asset_manager::AssetManagerPlugin);

    app.schedules.add(
        Schedules::Update,
        tests::move_camera::update_cam_system(Instant::now()),
    );
    app.schedules.add(
        Schedules::Update,
        tests::async_loading::setup_system(false),
    );
    app.schedules.add(
        Schedules::Update,
        tests::async_loading::update_uniforms_system(Instant::now())
    );

    app.run();
}
