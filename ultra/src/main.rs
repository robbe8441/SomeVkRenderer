use puddle::rendering::RenderPlugin;
use puddle::asset_manager::import_model;
use puddle::*;


fn main() {
    let mut app = Application::new();
    app.add_plugin(window::WindowPlugin);
    app.add_plugin(RenderPlugin);

    let vertecies = import_model!("./code.txt");
    dbg!(vertecies);
}
