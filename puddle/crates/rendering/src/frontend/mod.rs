use application::{PreUpdate, Startup, Update};

mod draw;
mod setup;
pub mod types;

pub use setup::PipelineSetup;

pub fn init(app: &mut application::Application) {
    app.add_systems(Startup, types::Camera::setup_system);

    app.add_systems(PreUpdate, setup::setup_pipeline);
    app.add_systems(Update, draw::draw);
}

