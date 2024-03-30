#![allow(unused, dead_code)]

mod hot_reload;
mod material;
mod model;
pub mod camera;

use camera::Camera;
pub use hot_reload::HotReloading;
pub use material::Material;
pub use model::{Model, ModelBuilder, load_model, model_from_string};

use application::{
    async_std::task, log::{error, warn}, Scheddules
};
use rendering::Renderer;
use std::{
    io::{Read, Seek},
    str::FromStr,
    time::Duration,
};

use std::fs;

pub struct AssetManagerPlugin;

impl application::Plugin for AssetManagerPlugin {
    fn finish(&mut self, app: &mut application::Application) {

        app.scheddules.add(
            Scheddules::UpdateEvery(Duration::from_secs(2)),
            hot_reload::check_updates_system(),
        );

        app.scheddules.add_non_parralel(Scheddules::Startup, |world, resources| {

            let renderer = resources.get::<rendering::Renderer>().expect("failed to crate camera: renderer does not exist");

            let cam = Camera::new(&renderer);

            drop(renderer);

            resources.insert(cam);
        });

        app.scheddules
            .add_non_parralel(Scheddules::Update, |world, resources| {

                let mut renderer = match resources.get_mut::<Renderer>() {
                    Some(r) => r,
                    None => {
                        warn!("cant find renderer");
                        return;
                    }
                };
                let mut camera = match resources.get::<Camera>() {
                    Some(r) => r,
                    None => {
                        warn!("cant find renderer");
                        return;
                    }
                };

                let mut context = match renderer.create_render_context() {
                    Some(r) => r,
                    None => return,
                };

                context.add_renderpass(rendering::RenderPass::ClearColor {
                    color: [1.0, 1.0, 0.5, 1.0],
                });

                use application::legion::query::*;
                let mut models = <(&Model, &HotReloading)>::query();

                for (model, _) in models.iter(world) {
                    context.add_renderpass(rendering::RenderPass::DrawIndexed {
                        vertex_buffer: &model.vertex_buffer,
                        index_buffer: &model.index_buffer,
                        pipeline: &model.material.pipeline,
                        bind_groups : &vec![&camera.bind_group],
                    });
                }

                context.flush(&mut renderer);
            });
    }
}

