use crate::material::Material;
use application::log::error;
use rendering::Buffer;

use std::sync::Arc;

pub struct Model {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub material: Arc<Material>,
}

#[derive(Debug)]
pub struct ModelBuilder {
    pub vertecies: Vec<rendering::Vertex>,
    pub indecies: Vec<u16>,
}

impl ModelBuilder {
    pub fn build(self, renderer: &rendering::Renderer, material: Arc<Material>) -> Model {

        use rendering::wgpu::BufferUsages;
        let vertex_buffer = renderer.create_buffer(BufferUsages::VERTEX, &self.vertecies);
        let index_buffer = renderer.create_buffer(BufferUsages::INDEX, &self.indecies);

        Model {
            vertex_buffer,
            index_buffer,
            material,
        }
    }
}
