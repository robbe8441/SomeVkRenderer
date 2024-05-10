use std::sync::Arc;

use bevy_ecs::system::{Commands, Res};
use bevy_ecs::world::World;
use components::Transform;
use glam::{Mat4, Vec3};
use vulkano::buffer::{allocator::SubbufferAllocator, BufferUsage};
use vulkano::buffer::{BufferContents, Subbuffer};
use vulkano::memory::allocator::{MemoryTypeFilter, StandardMemoryAllocator};

use crate::backend;

pub struct Camera {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,

    pub transform: components::Transform,

    allocator: SubbufferAllocator,
}

#[repr(C)]
#[derive(BufferContents)]
pub struct CameraUniform {
    proj: [[f32; 4]; 4],
    pos: [f32; 3],
}

impl Camera {
    pub fn build_proj(&self) -> Subbuffer<CameraUniform> {
        let view = Mat4::look_at_rh(
            self.transform.translation * Vec3::new(1.0, -1.0, 1.0),
            self.transform.forward(),
            self.transform.up(),
        );

        let proj =
            Mat4::perspective_rh_gl(self.fovy.to_radians(), self.aspect, self.znear, self.zfar);

        let uniform_data = CameraUniform {
            proj: (proj * view).to_cols_array_2d(),
            pos: self.transform.translation.to_array(),
        };

        let subbuffer = self.allocator.allocate_sized().unwrap();
        *subbuffer.write().unwrap() = uniform_data;

        subbuffer
    }

    pub fn new(memory_allocator: Arc<StandardMemoryAllocator>, screen_size: [f32; 2]) -> Self {
        let allocator = SubbufferAllocator::new(
            memory_allocator.clone(),
            vulkano::buffer::allocator::SubbufferAllocatorCreateInfo {
                buffer_usage: BufferUsage::UNIFORM_BUFFER,
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
        );

        let aspect = screen_size[0] / screen_size[1];

        Self {
            transform: Transform::default(),
            aspect,
            fovy: 60.0,
            znear: 0.1,
            zfar: 100.0,
            allocator,
        }
    }

    pub fn setup_system(
        memory_allocator: Res<backend::buffer::StandardMemoryAllocator>,
        window: Res<window::Window>,
        mut commands: Commands,
    ) {
        let cam = Camera::new(memory_allocator.0.clone(), window.0.inner_size().into());

        commands.add(|world: &mut World| {
            world.insert_non_send_resource(cam);
        });
    }
}
