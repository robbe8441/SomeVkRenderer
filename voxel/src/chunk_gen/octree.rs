#![allow(dead_code, unused)]

use cgmath::{Vector3 as vec3, Zero};
use std::sync::{Arc, Mutex};

pub type Vector3 = vec3<f32>;
pub type Vector3i = vec3<i32>;
pub type Color = vec3<f32>;

type MutNode = Arc<Mutex<Node>>;

#[derive(Debug)]
struct VoxelData {
    color: Color,
}

#[derive(Debug)]
struct VoxelInfo {
    color: Color,
}

#[derive(Debug)]
struct Node {
    is_leaf: bool,
    children: [Option<Arc<Mutex<Node>>>; 8],
    data: VoxelData,
}

impl Node {
    fn new() -> Self {
        Self {
            is_leaf: false,
            data: VoxelData {
                color: Color::zero(),
            },
            children: [None, None, None, None, None, None, None, None],
        }
    }
}

#[derive(Debug)]
pub struct SparseVoxelOctree {
    size: i32,
    max_depth: i32,
    root: Arc<Mutex<Node>>,
}

impl SparseVoxelOctree {
    pub fn new(size: i32, max_depth: i32) -> Self {
        let root = Arc::new(Mutex::new(Node::new()));

        Self {
            size,
            max_depth,
            root,
        }
    }

    pub fn insert(&mut self, pos: &Vector3i, color: &Color) {
        self.insert_internal(
            self.root.clone(),
            pos,
            color,
            &mut Vector3i { x: 0, y: 0, z: 0 },
            0,
        );
    }

    fn insert_internal(
        &mut self,
        node: Arc<Mutex<Node>>,
        point: &Vector3i,
        color: &Color,
        position: &mut Vector3i, // position inside the octree ( do not set manually )
        depth: i32,
    ) {
        let mut mut_node = node.lock().unwrap();

        mut_node.data.color = color.clone();

        if depth == self.max_depth {
            mut_node.is_leaf = true;
            return;
        }

        let size = self.size as f32 / (depth as f32).exp2();

        let child_pos = Vector3i {
            x: (point.x as f32 >= (size * position.x as f32) + (size / 2.0)) as i32,
            y: (point.y as f32 >= (size * position.y as f32) + (size / 2.0)) as i32,
            z: (point.z as f32 >= (size * position.z as f32) + (size / 2.0)) as i32,
        };

        let child_index: i32 = (child_pos.x << 0) | (child_pos.y << 1) | (child_pos.z << 2);

        *position = Vector3i {
            x: (position.x << 1) | child_pos.x,
            y: (position.y << 1) | child_pos.y,
            z: (position.z << 1) | child_pos.z,
        };

        let child_node = &mut mut_node.children[child_index as usize];

        if child_node.is_none() {
            *child_node = Some(Arc::new(Mutex::new(Node::new())));
        }

        let child = mut_node.children.as_ref()[child_index as usize]
            .clone()
            .unwrap();

        self.insert_internal(child, point, color, position, depth + 1);
    }
}
