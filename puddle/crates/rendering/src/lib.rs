#![allow(unused, dead_code)]

mod backend;
mod frontend;

pub struct RenderPlugin;

impl application::Plugin for RenderPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        let backend = backend::Renderer::new(app);

        app.resources.insert(backend);
        app.scheddules
            .add_non_parralel(application::Scheddules::Update, frontend::test::init);
    }
}
