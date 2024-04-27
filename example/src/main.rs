#![allow(unused, dead_code)]

use std::time::{Duration, Instant};

use application::Schedules;
use puddle::*;

struct DeltaTime(f32);

fn main() {
    let mut app = puddle::application::Application::new();

    app.add_plugin(window::WindowPlugin);
    app.add_plugin(rendering::RenderPlugin);
    //app.add_plugin(asset_manager::AssetManagerPlugin);
    
    app.schedules.add(Schedules::Update, record_deltatime_system(Instant::now()));
    app.schedules.add(Schedules::UpdateEvery(Duration::from_secs(1)), print_delta_system());

    app.resources.insert(DeltaTime(0.0));

    app.run();
}

#[legion::system]
fn record_deltatime(
    #[state] last_update : &mut Instant,
    #[resource] dt : &mut DeltaTime,
    ) {
    dt.0 = last_update.elapsed().as_secs_f32();
    *last_update = Instant::now();
}

#[legion::system]
fn print_delta(
    #[resource] dt : &mut DeltaTime,
    ) {

    println!("fps : {}",1.0 / dt.0);

}
