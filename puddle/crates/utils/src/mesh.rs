use std::sync::Arc;

use super::Vertex;

#[allow(unused)]
pub struct Mesh {
    pub vertecies: Arc<[Vertex]>,
    pub indices: Arc<[u16]>,
}

impl Mesh {
    pub fn plane() -> Self {
        let vertecies = vec![
            Vertex {
                position: [-0.5, -0.5, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.0],
            },
        ];

        let indices = vec![0, 1, 3, 3, 2, 0];

        Self {
            vertecies: vertecies.into(),
            indices: indices.into(),
        }
    }
    pub fn cube() -> Self {
        let vertecies = vec![
            Vertex {
                position: [-1.0, -1.0, -1.0],
            },
            Vertex {
                position: [1.0, -1.0, -1.0],
            },
            Vertex {
                position: [1.0, 1.0, -1.0],
            },
            Vertex {
                position: [-1.0, 1.0, -1.0],
            },
            Vertex {
                position: [-1.0, -1.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [-1.0, 1.0, 1.0],
            },
        ];

        let indices = vec![
            0, 1, 2, 0, 2, 3, 1, 5, 6, 1, 6, 2, 4, 5, 6, 4, 6, 7, 0, 4, 7, 0, 7, 3, 3, 2, 6, 3, 6,
            7, 0, 1, 5, 0, 5, 4,
        ];

        Self {
            vertecies: vertecies.into(),
            indices: indices.into(),
        }
    }
}
