use puddle::*;
use bevy_ecs::system::Res;

fn main() {
    let mut app = puddle::application::Application::default();

    app.add_plugin(window::WindowPlugin);
    app.add_plugin(rendering::RenderPlugin);
    app.add_plugin(time::TimePlugin);

    app.add_systems(application::Update, print_delta);

    app.run();
}

fn print_delta(time: Res<time::Time>) {
    println!("fps : {}", 1.0 / time.delta64);
}
