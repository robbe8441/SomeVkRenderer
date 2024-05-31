use std::time::Instant;

use bevy_ecs::system::{Res, ResMut, Resource};
use glam::FloatExt;
use puddle::*;

fn main() {
    let mut app = Application::default();

    app.add_plugin(window::WindowPlugin);
    app.add_plugin(rendering::RenderPlugin);

    app.add_plugin(time::TimePlugin);

    app.add_systems(application::Update, print_delta);

    app.world.insert_resource(AverageFPS(0.0, Instant::now()));

    app.run();
}

#[derive(Resource)]
struct AverageFPS(f32, Instant);

fn print_delta(time: Res<time::Time>, mut avg: ResMut<AverageFPS>) {
    let fps = 1.0 / time.delta;

    avg.0 = avg.0.lerp(fps, 10.0 * time.delta);

    if avg.1.elapsed().as_secs() > 2 {
        println!("fps : {}", avg.0.floor());
        avg.1 = Instant::now();
    }
}
