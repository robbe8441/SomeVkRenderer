#![allow(unused, dead_code)]

use std::time::{Duration, Instant};

use application::Schedules;
use puddle::*;

struct DeltaTime(f32);

fn main() {
    let mut app = puddle::application::Application::new();

    app.add_plugin(window::WindowPlugin);
    app.add_plugin(rendering::RenderPlugin);
    app.add_plugin(time::TimePlugin);

    app.schedules.add(
        Schedules::UpdateEvery(Duration::from_secs(1)),
        print_delta_system(),
    );

    app.resources.insert(DeltaTime(0.0));

    app.run();
}

#[legion::system]
fn print_delta(#[resource] time: &time::Time) {
    println!("fps : {:.0}", 1.0 / time.delta);
}
