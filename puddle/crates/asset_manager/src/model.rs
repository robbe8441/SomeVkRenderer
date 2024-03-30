use crate::{material::Material, HotReloading};
use application::log::error;
use rendering::{wgpu::hal::auxil::db, Buffer};

use std::{fs, io::Read, path::Path, str::FromStr, sync::Arc};

#[derive(Clone)]
pub struct Model {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub material: Arc<Material>,
}

#[derive(Debug)]
pub struct ModelBuilder {
    pub vertecies: Vec<rendering::Vertex>,
    pub indecies: Vec<u32>,
    pub file_path: Option<String>,
}

impl ModelBuilder {
    pub fn get_hot_reload(&self) -> std::io::Result<HotReloading> {
        Ok(HotReloading::new(
            self.file_path.clone().expect("no given file path").as_str(),
        )?)
    }

    pub fn build(self, renderer: &rendering::Renderer, material: Arc<Material>) -> Model {
        use rendering::wgpu::BufferUsages;
        let vertex_buffer = renderer.create_buffer(
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
            &self.vertecies,
        );
        let index_buffer =
            renderer.create_buffer(BufferUsages::INDEX | BufferUsages::COPY_DST, &self.indecies);

        Model {
            vertex_buffer,
            index_buffer,
            material,
        }
    }
}

fn parse_numbers<T: Default + FromStr>(input: &str) -> Vec<T>
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

pub fn load_model(path: &Path) -> ModelBuilder {
    let mut string = String::new();
    let file = fs::OpenOptions::new()
        .read(true)
        .open(path)
        .and_then(|mut file| file.read_to_string(&mut string));

    let mut model = model_from_string(&string);
    model.file_path = path.to_str().map(|r| r.to_string());
    model
}

pub fn model_from_string(input: &str) -> ModelBuilder {
    let mut model = ModelBuilder {
        vertecies: vec![],
        indecies: vec![],
        file_path: None,
    };

    use regex::Regex;
    use rendering::Vertex;

    // best website everrr https://regexr.com/
    let positions = Regex::new(r"v( -?(\d+)?\.\d+){3}").unwrap();
    let normals = Regex::new(r"vn( -?(\d+)?\.\d+){3}").unwrap();
    let uv_cords = Regex::new(r"vt( -?(\d+)?\.\d+){2}").unwrap();

    // pos, uv, normal
    let faces = Regex::new(r"f( \d+\/\d+\/\d+){3}").unwrap();

    let mut vertex_positions: Vec<[f32; 3]> = vec![];
    let mut vertex_normals: Vec<[f32; 3]> = vec![];
    let mut vertex_uvs: Vec<[f32; 2]> = vec![];

    // get vertecies
    for v in positions.find_iter(input) {
        let vert = v.as_str().replace("v ", "");
        let pos: [f32; 3] = parse_numbers(&vert).try_into().unwrap();
        vertex_positions.push(pos);
    }

    // get normals
    for v in normals.find_iter(input) {
        let vert = v.as_str().replace("vn ", "");
        let pos: [f32; 3] = parse_numbers(&vert).try_into().unwrap();
        vertex_normals.push(pos);
    }

    // get uv's
    for v in uv_cords.find_iter(input) {
        let vert = v.as_str().replace("vt ", "");
        let pos: [f32; 2] = parse_numbers(&vert).try_into().unwrap();
        vertex_uvs.push(pos);
    }

    // load triangles
    for v in faces.find_iter(input) {
        let vert = v.as_str().replace("f ", "");

        for vertex_data in vert.split_whitespace() {
            let vertex_data = vertex_data.replace("/", " ");
            let data: [usize; 3] = parse_numbers(&vertex_data).try_into().unwrap();

            model.vertecies.push(Vertex {
                position: vertex_positions[data[0] - 1],
                uv_cords: vertex_uvs[data[1] - 1],
                normal: vertex_normals[data[2] - 1],
            });

            model.indecies.push(model.vertecies.len() as u32 - 1);
        }
    }

    model
}
