mod camera;
mod view;
use std::time::Instant;

use puddle::application::{Application, Scheddules};

fn main() {
    let mut app = Application::new();
    app.add_plugin(puddle::window::WindowPlugin);
    app.add_plugin(puddle::rendering::RenderPlugin);

    app.scheddules
        .add(Scheddules::Startup, view::add_view_system());
    app.scheddules.add(
        Scheddules::Update,
        view::update_uniforms_system(Instant::now()),
    );
    app.scheddules
        .add_non_parralel(Scheddules::Update, camera::camera_controller);

    app.run();
}
