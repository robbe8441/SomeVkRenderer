pub mod backend;
pub use vulkano;

pub struct RenderPlugin;

impl application::Plugin for RenderPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        backend::init(app);
    }
}
