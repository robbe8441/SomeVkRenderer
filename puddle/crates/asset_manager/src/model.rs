use bevy_ecs::{bundle::Bundle, component::Component, system::Commands};
use components::Transform;
use rendering::{
    backend::buffer::StandardMemoryAllocator,
    frontend::types::Vertex3D,
};

#[derive(Component, Default, Debug)]
pub struct Vertices(pub Vec<Vertex3D>);

#[derive(Bundle, Default)]
pub struct ModelBundle {
    transform: Transform,
    vertices: Vertices,
}

use bevy_ecs::system::Res;
pub fn load_vertex_buffer(memory_allocator: Res<StandardMemoryAllocator>, mut commands: Commands) {
    use rendering::frontend::types::VertexBuffer;

    let buffer = VertexBuffer::new(&memory_allocator, &Vertices::cube().0);

    commands.insert_resource(buffer);
}

impl Vertices {
    pub fn cube() -> Self {
        Self(vec![
            // Front face
            Vertex3D {
                position: [-0.5, -0.5, -0.5],
            },
            Vertex3D {
                position: [0.5, -0.5, -0.5],
            },
            Vertex3D {
                position: [0.5, 0.5, -0.5],
            },
            Vertex3D {
                position: [0.5, 0.5, -0.5],
            },
            Vertex3D {
                position: [-0.5, 0.5, -0.5],
            },
            Vertex3D {
                position: [-0.5, -0.5, -0.5],
            },
            // Back face
            Vertex3D {
                position: [0.5, -0.5, 0.5],
            },
            Vertex3D {
                position: [-0.5, -0.5, 0.5],
            },
            Vertex3D {
                position: [-0.5, 0.5, 0.5],
            },
            Vertex3D {
                position: [-0.5, 0.5, 0.5],
            },
            Vertex3D {
                position: [0.5, 0.5, 0.5],
            },
            Vertex3D {
                position: [0.5, -0.5, 0.5],
            },
            // Left face
            Vertex3D {
                position: [-0.5, -0.5, 0.5],
            },
            Vertex3D {
                position: [-0.5, -0.5, -0.5],
            },
            Vertex3D {
                position: [-0.5, 0.5, -0.5],
            },
            Vertex3D {
                position: [-0.5, 0.5, -0.5],
            },
            Vertex3D {
                position: [-0.5, 0.5, 0.5],
            },
            Vertex3D {
                position: [-0.5, -0.5, 0.5],
            },
            // Right face
            Vertex3D {
                position: [0.5, -0.5, -0.5],
            },
            Vertex3D {
                position: [0.5, -0.5, 0.5],
            },
            Vertex3D {
                position: [0.5, 0.5, 0.5],
            },
            Vertex3D {
                position: [0.5, 0.5, 0.5],
            },
            Vertex3D {
                position: [0.5, 0.5, -0.5],
            },
            Vertex3D {
                position: [0.5, -0.5, -0.5],
            },
            // Top face
            Vertex3D {
                position: [-0.5, 0.5, -0.5],
            },
            Vertex3D {
                position: [0.5, 0.5, -0.5],
            },
            Vertex3D {
                position: [0.5, 0.5, 0.5],
            },
            Vertex3D {
                position: [0.5, 0.5, 0.5],
            },
            Vertex3D {
                position: [-0.5, 0.5, 0.5],
            },
            Vertex3D {
                position: [-0.5, 0.5, -0.5],
            },
            // Bottom face
            Vertex3D {
                position: [-0.5, -0.5, 0.5],
            },
            Vertex3D {
                position: [0.5, -0.5, 0.5],
            },
            Vertex3D {
                position: [0.5, -0.5, -0.5],
            },
            Vertex3D {
                position: [0.5, -0.5, -0.5],
            },
            Vertex3D {
                position: [-0.5, -0.5, -0.5],
            },
            Vertex3D {
                position: [-0.5, -0.5, 0.5],
            },
        ])
    }
}
