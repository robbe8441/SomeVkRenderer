use puddle::rendering::RenderPlugin;
use puddle::*;

fn main() {
    let mut app = Application::new();
    app.add_plugin(window::WindowPlugin);
    app.add_plugin(RenderPlugin);

    app.run();
}

