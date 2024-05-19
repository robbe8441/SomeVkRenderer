use application::{PreUpdate, Startup, Update};

mod draw;
mod setup;
mod instancing;
pub mod types;

pub use setup::PipelineSetup;
pub use instancing::InstancingTransforms;

pub fn init(app: &mut application::Application) {
    app.add_systems(Startup, types::Camera::setup_system);

    app.add_systems(PreUpdate, (setup::setup_pipeline, instancing::reload_instance_buffer));
    app.add_systems(Update, draw::draw);
}

