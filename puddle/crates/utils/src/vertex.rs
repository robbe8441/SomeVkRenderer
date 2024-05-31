use glam::Vec3;
use vulkano::{buffer::BufferContents,  pipeline::graphics::vertex_input};

#[repr(C)]
#[derive(vertex_input::Vertex, Debug, Clone, Copy, Default, BufferContents)]
pub struct Vertex {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],
}

#[allow(unused, dead_code)]
impl Vertex {

    fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: [x, y, z],
        }
    }

    fn from_pos(pos: Vec3) -> Self {
        Self {
            position: pos.into(),
        }
    }
}
