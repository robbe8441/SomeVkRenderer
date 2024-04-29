mod setup;
mod draw;
mod instances;

pub struct RenderPlugin;

 

impl application::Plugin for RenderPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        setup::init(app);
        app.schedules.add_non_parallel(application::Schedules::Update, draw::draw);
    }
}
