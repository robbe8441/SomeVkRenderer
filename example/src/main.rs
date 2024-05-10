use std::time::Instant;

use bevy_ecs::system::{Commands, NonSendMut, Res, ResMut, Resource};
use puddle::asset_manager::Vertices;
use puddle::*;

fn main() {
    let mut app = puddle::application::Application::default();

    app.add_plugin(window::WindowPlugin);
    app.add_plugin(rendering::RenderPlugin);
    app.add_plugin(asset_manager::AssetManagerPlugin);
    app.add_plugin(time::TimePlugin);

    app.add_systems(application::Update, (print_delta, update_cam));

    app.add_systems(application::Startup, load_model);

    app.world
        .insert_resource(AverageFPS(2000.0, Instant::now()));

    app.run();
}

fn load_model(mut commands: Commands) {
    let model = Vertices::cube();
    commands.spawn(model);
}

use puddle::rendering::frontend::types::Camera;




fn update_cam(mut cam: NonSendMut<Camera>, time: Res<time::Time>) {
    let t = time.startup.elapsed().as_secs_f32();
    use components::Transform;
    use glam::Vec3;

    let pos = Vec3::new(t.sin(), t.cos() / 10.0, t.cos()) * 2.0;
    cam.transform = Transform::from_translation(pos).looking_at(Vec3::ZERO, Vec3::Y);
}

#[derive(Resource)]
struct AverageFPS(f32, Instant);

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn print_delta(time: Res<time::Time>, mut avg: ResMut<AverageFPS>) {
    let fps = 1.0 / time.delta;

    avg.0 = lerp(avg.0, fps, 10.0 * time.delta);

    if avg.1.elapsed().as_secs() > 2 {
        println!("fps : {}", avg.0.floor());
        avg.1 = Instant::now();
    }
}
