#![allow(unused, dead_code)]

mod material;
mod model;
mod hot_reload;

pub use material::Material;
pub use model::{Model, ModelBuilder};
pub use hot_reload::HotReloading;
use rendering::Renderer;

use std::str::FromStr;

use application::log::{error, warn};
pub use regex;

pub struct AssetManagerPlugin;

impl application::Plugin for AssetManagerPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        hot_reload::init(app);

        use application::Scheddules;

        app.scheddules
            .add_non_parralel(Scheddules::Update, |world, resources| {
                let mut renderer = match resources.get_mut::<Renderer>() {
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
                let mut models = <&Model>::query();

                for model in models.iter(world) {
                    context.add_renderpass(rendering::RenderPass::DrawIndexed {
                        vertex_buffer: &model.vertex_buffer,
                        index_buffer: &model.index_buffer,
                        pipeline: &model.material.pipeline,
                    });
                }

                context.flush(&mut renderer);
            });
    }
}

pub fn parse_numbers<T: Default + FromStr>(input: String) -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    let mut res = vec![];

    for num in input.split_whitespace() {
        let parsed = match num.parse() {
            Ok(r) => r,
            Err(e) => {
                error!("Failed parsing number when loading model : {}", e);
                continue;
            }
        };
        res.push(parsed);
    }

    res
}

pub fn model_from_string(input: &str) -> ModelBuilder {
    let mut model = ModelBuilder {
        vertecies: vec![],
        indecies: vec![],
    };

    use regex::Regex;
    use rendering::Vertex;

    // best website everrr https://regexr.com/
    let vertecies = Regex::new(r"v( -?\d+.\d+){3}").unwrap();
    let uv_cords = Regex::new(r"u( -?\d+.\d+){2}").unwrap();
    let indecies = Regex::new(r"i( \d+){3}").unwrap();

    for v in vertecies.find_iter(input) {
        let vert = v.as_str().replace("v ", "");
        model.vertecies.push(Vertex {
            position: parse_numbers(vert).try_into().unwrap(),
            uv_cords: [0.0, 0.0],
        });
    }

    for (i, v) in uv_cords.find_iter(input).enumerate() {
        let vert = v.as_str().replace("u ", "");

        let mut vertex = match model.vertecies.get_mut(i) {
            Some(r) => r,
            None => {
                error!("error when indexing uv_cords when loading a model",);
                continue;
            }
        };

        vertex.uv_cords = parse_numbers(vert).try_into().unwrap();
    }

    for line in indecies.find_iter(input) {
        let face = line.as_str().replace("i ", "");

        let numbers: [u16; 3] = parse_numbers(face).try_into().unwrap();

        model.indecies.extend(numbers.iter());
    }

    model
}

#[macro_export]
macro_rules! import_model {
    ($path:expr) => {{
        let model = include_str!($path);
        puddle::asset_manager::model_from_string(model)
    }};
}
