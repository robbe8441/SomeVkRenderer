#![allow(unused, dead_code)]
mod hot_reload;
mod model;

pub use hot_reload::HotReloading;
pub use model::{load_model, model_from_string, AsyncModelBuilder, AsyncModelQueue};
use rendering::{
    utils::{Camera, Material, Model},
    Renderer,
};

use application::{
    log::{error, warn},
    Schedules,
};
use std::{
    fs,
    io::{Read, Seek},
    str::FromStr,
    time::Duration,
    vec,
};

pub struct AssetManagerPlugin;

impl application::Plugin for AssetManagerPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        app.schedules.add(
            Schedules::UpdateEvery(Duration::from_secs(2)),
            hot_reload::check_updates_system(),
        );

        app.schedules
            .add(Schedules::Update, model::load_model_queue_system(vec![]));

        app.resources.insert(model::AsyncModelQueue(vec![]));
    }
}
