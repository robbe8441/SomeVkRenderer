mod octree;
use octree::{Color, SparseVoxelOctree, Vector3i};

pub fn test() {
    let mut octree = SparseVoxelOctree::new(32, 3);
    octree.insert(&Vector3i::new(0, 0, 0), &Color::new(1.0, 0.0, 0.0));

    dbg!(octree);
}
