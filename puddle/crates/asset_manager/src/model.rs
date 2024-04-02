use application::log::error;
use legion::systems::CommandBuffer;
use rendering::{
    utils::{Material, Model, MeshAsset},
    wgpu, Vertex,
};

use std::{collections::HashMap, fs::OpenOptions, io::Read, path::Path, str::FromStr, sync::{Arc, Mutex}, thread};

type OnLoadedCallBack =
    Arc<dyn Fn(legion::Entity, &mut ModelBuilder, &mut CommandBuffer) + Send + Sync>;

pub struct AsyncModelBuilder {
    pub path: String,
    pub material: Arc<Material>,
    on_loaded: OnLoadedCallBack,
}

impl AsyncModelBuilder {
    pub fn new(path: String, material: Arc<Material>) -> Self {
        Self {
            path,
            material,
            on_loaded: Arc::new(|_, _, _|{}),
        }
    }

    #[inline]
    pub fn and_then<F>(mut self, f: F) -> Self
    where
        F: Fn(legion::Entity, &mut ModelBuilder, &mut CommandBuffer)
            + std::marker::Send
            + std::marker::Sync
            + 'static,
    {
        self.on_loaded = Arc::new(f);
        self
    }
}

pub struct AsyncModelQueue(pub Vec<AsyncModelBuilder>);

impl AsyncModelQueue {
    #[inline(always)]
    pub fn push(&mut self, model: AsyncModelBuilder) {
        self.0.push(model);
    }
}

#[derive(Debug)]
pub struct ModelBuilder {
    pub vertecies: Vec<Vertex>,
    pub indecies: Vec<u32>,
    pub file_path: Option<String>,
}

impl Default for ModelBuilder {
    fn default() -> Self {
        Self {
            vertecies: vec![],
            indecies: vec![],
            file_path: None,
        }
    }
}

impl ModelBuilder {
    pub fn build(self, renderer: &crate::Renderer, material: Arc<Material>) -> MeshAsset {
        use wgpu::BufferUsages;
        let vertex_buffer = renderer.create_buffer(
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
            &self.vertecies,
        );
        let index_buffer =
            renderer.create_buffer(BufferUsages::INDEX | BufferUsages::COPY_DST, &self.indecies);

        let instance_buffer = 
            renderer.create_buffer(BufferUsages::VERTEX | BufferUsages::COPY_DST, &self.indecies);

        MeshAsset {
            vertex_buffer,
            index_buffer,
            instance_buffer,
            instances: Arc::new(Mutex::new(HashMap::new())),
            material,
        }
    }
}

#[legion::system]
pub fn load_model_queue(
    #[state] thread_pool: &mut Vec<
        thread::JoinHandle<(ModelBuilder, Arc<Material>, OnLoadedCallBack)>,
    >,
    #[resource] renderer: &rendering::Renderer,
    #[resource] loader: &mut AsyncModelQueue,
    mut commands: &mut CommandBuffer,
) {
    if let Some(model) = loader.0.pop() {
        thread_pool.push(thread::spawn(move || {
            let builder = load_model(Path::new(&model.path));
            (builder, model.material, model.on_loaded)
        }));
    }

    for i in (0..thread_pool.len()).rev() {
        if thread_pool[i].is_finished() {
            let thread = thread_pool.remove(i);

            thread
                .join()
                .and_then(|(mut model, material, callback)| {
                    println!("loaded model");

                    let entt = commands.push(());

                    callback(entt, &mut model, &mut commands);

                    let model = model.build(renderer, material);

                    commands.add_component(entt, model);

                    Ok(())
                })
                .inspect_err(|_| {
                    error!("failed loading model async");
                });
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
                error!(
                    "Failed parsing number when loading model : {} number : {}",
                    e, num
                );
                continue;
            }
        };
        res.push(parsed);
    }
    res
}

pub fn load_model(path: &Path) -> ModelBuilder {
    let mut string = String::new();
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .and_then(|mut file| file.read_to_string(&mut string))
        .inspect_err(|e| error!("failed to load model {}", e));

    let mut model = model_from_string(&string);
    model.file_path = path.to_str().map(|r| r.to_string());
    model
}
pub fn model_from_string(input: &str) -> ModelBuilder {
    let mut model = ModelBuilder::default();

    let mut vertex_positions = Vec::new();
    let mut vertex_normals = Vec::new();
    let mut vertex_uvs = Vec::new();

    for (i, line) in input.lines().enumerate() {
        match &line[..2] {
            "v " => {
                let pos: [f32; 3] = parse_numbers(&line[2..])[0..3].try_into().unwrap();
                vertex_positions.push(pos);
            }
            "vn" => {
                let pos: [f32; 3] = parse_numbers(&line[2..])[0..3].try_into().unwrap();
                vertex_normals.push(pos);
            }
            "vt" => {
                let pos: [f32; 2] = parse_numbers(&line[2..])[0..2].try_into().unwrap();
                vertex_uvs.push(pos);
            }
            "f " => {
                let face_data = &line[2..].replace("/", " ");
                //println!("{:?} in line {}", face_data, i);
                let data: [usize; 9] = match parse_numbers(face_data)[0..9].try_into() {
                    Ok(r) => r,
                    Err(e) => {
                        error!("failed to laod face : {}", e);
                        continue;
                    }
                };

                let len = model.vertecies.len();

                let mut vertecies_to_add = vec![];

                for i in 0..3 {
                    let face_data = &data[i * 3..];
                    vertecies_to_add.push(Vertex {
                        position: vertex_positions[face_data[0] - 1],
                        uv_cords: vertex_uvs[face_data[1] - 1],
                        normal: vertex_normals[face_data[2] - 1],
                    });
                }

                model.vertecies.extend_from_slice(&vertecies_to_add);
                model.indecies.extend((len as u32)..(len + 3) as u32);
            }
            _ => {}
        }
    }

    model
}
