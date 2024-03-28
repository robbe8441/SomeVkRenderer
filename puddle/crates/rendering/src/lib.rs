#![allow(unused, dead_code)]

mod backend;
mod frontend;
mod types;

pub struct RenderPlugin;
pub use types::Vertex;
pub use backend::Buffer;
pub use wgpu::RenderPipeline;

impl application::Plugin for RenderPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        let backend = backend::Renderer::new(app);

        app.resources.insert(backend);

        app.scheddules
            .add_non_parralel(application::Scheddules::Update, frontend::test::init);
        app.scheddules
            .add_non_parralel(application::Scheddules::Startup, frontend::test::srtup);
    }
}
