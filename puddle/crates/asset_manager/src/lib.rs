#![allow(unused, dead_code)]
mod hot_reload;
mod model;

pub use model::{load_model, model_from_string, AsyncModelQueue, AsyncModelBuilder};

use rendering::utils::{Material, Camera, Model};
pub use hot_reload::HotReloading;

use application::{
    async_std::task, log::{error, warn}, Schedules
};
use rendering::Renderer;
use std::{
    io::{Read, Seek},
    str::FromStr,
    time::Duration, vec,
}; 
 
use std::fs;

pub struct AssetManagerPlugin;

impl application::Plugin for AssetManagerPlugin {
    fn finish(&mut self, app: &mut application::Application) {

        app.schedules.add(
            Schedules::UpdateEvery(Duration::from_secs(2)),
            hot_reload::check_updates_system(),
        );

        app.schedules.add(Schedules::Update, model::load_model_queue_system(vec![]));

        app.resources.insert(model::AsyncModelQueue(vec![]));


    }
}

