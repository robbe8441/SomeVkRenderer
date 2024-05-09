pub mod backend;
pub mod frontend;


pub struct RenderPlugin;

impl application::Plugin for RenderPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        backend::init(app);
        frontend::init(app);
    }
}
