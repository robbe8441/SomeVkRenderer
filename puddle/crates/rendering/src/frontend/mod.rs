use application::{PreUpdate, Startup, Update};

pub mod types;
mod draw;
mod setup;

pub fn init(app: &mut application::Application) {
    app.add_systems(Startup, types::Camera::setup_system);

    app.add_systems(PreUpdate, setup::setup_pipeline);
    app.add_systems(Update, draw::draw);

}
