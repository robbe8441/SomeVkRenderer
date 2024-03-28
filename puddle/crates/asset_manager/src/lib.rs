#![allow(unused, dead_code)]


use std::str::FromStr;

use application::log::error;
use rendering::Buffer;
pub use regex;

pub struct Material {
    pipeline : rendering::RenderPipeline,
}

pub struct Model {
    vertex_buffer : Buffer,
    index_buffer : Buffer,
    material : Material,
}

#[derive(Debug)]
pub struct ModelBuilder {
    pub vertecies : Vec<rendering::Vertex>,
    pub indecies : Vec<u16>,
}



pub fn parse_numbers<T: Default + FromStr>(input : String) -> Vec<T> 
    where <T as FromStr>::Err : std::fmt::Display
{
    let mut res = vec![];

    for num in input.split_whitespace() {
        let parsed = match num.parse() {
            Ok(r) => r,
            Err(e) => { error!("Failed parsing number when loading model : {}", e); continue; }
        };
        res.push(parsed);
    }

    res
}


#[macro_export]
macro_rules! import_model {
    ($path:expr) => {{

        use puddle::asset_manager::regex::Regex;
        use puddle::rendering::Vertex;
        use puddle::application::log::error;

        use puddle::asset_manager::ModelBuilder;

        let mut model = ModelBuilder {
            vertecies : vec![],
            indecies : vec![],
        };


        let file = include_str!($path);

        // best website everrr https://regexr.com/
        let vertecies = Regex::new(r"v( \d+.\d+){3}").unwrap();
        let uv_cords = Regex::new(r"u( \d+.\d+){2}").unwrap();
        let indecies = Regex::new(r"i( \d+){3}").unwrap();

        for v in vertecies.find_iter(file) {
            let vert = v.as_str().replace("v ", "");
            model.vertecies.push(Vertex {
                position: puddle::asset_manager::parse_numbers(vert).try_into().unwrap(),
                uv_cords: [0.0, 0.0],
            });
        }

        for (i,v) in uv_cords.find_iter(file).enumerate() {
            let vert = v.as_str().replace("u ", "");

            let mut vertex = match model.vertecies.get_mut(i) {
                Some(r) => r,
                None => { error!("error when indexing uv_cords when loading a model : {}", $path); continue; }
            };

            vertex.uv_cords = puddle::asset_manager::parse_numbers(vert).try_into().unwrap();
        }

        for line in indecies.find_iter(file) {
            let face = line.as_str().replace("i ", "");

            let numbers : [u16; 3] = puddle::asset_manager::parse_numbers(face).try_into().unwrap();

            model.indecies.extend(numbers.iter());
        }


        model
    }};
}

