use components::Transform;
use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

#[repr(C)]
#[derive(BufferContents, Vertex, Clone, Debug)]
pub struct InstanceData {
    #[format(R32G32B32_SFLOAT)]
    pub instance_position: [f32; 3],
}

impl From<Transform> for InstanceData {
    fn from(value: Transform) -> Self {
        Self {
            instance_position: value.translation.to_array(),
        }
    }
}
