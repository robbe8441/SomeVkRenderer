use cgmath::Vector3;
use rendering::{
    wgpu::{BufferUsages, ShaderStages},
    Buffer,
};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

pub struct CameraData {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

pub struct Camera {
    pub bind_group: rendering::wgpu::BindGroup,
    pub bind_group_layout: rendering::wgpu::BindGroupLayout,
    pub uniform_buffer: rendering::Buffer,
    pub camera_uniform : CameraUniform,
    pub data: CameraData,
}

impl Camera {
    pub fn new(renderer: &rendering::Renderer) -> Self {
        let data = CameraData::default();

        let camera_uniform = CameraUniform {
            view_proj: data.build_view_projection_matrix().into(),
        };

        let uniform_buffer = renderer.create_buffer(
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            &[camera_uniform],
        );

        let layout = rendering::BindGroupLayout {
            entries: vec![rendering::BindGroupLayoutEntry {
                visibility: ShaderStages::VERTEX,
                ty: rendering::BindingType::Buffer(rendering::wgpu::BufferBindingType::Uniform),
                resource: uniform_buffer.binding(),
            }],
        };

        let (bind_group, bind_group_layout) = layout.build(renderer);

        Self {
            bind_group,
            bind_group_layout,
            uniform_buffer,
            camera_uniform,
            data,
        }
    }
}
impl CameraData {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    pub fn default() -> Self {
        Self {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: 1.0, // TODO : use width divided by hight
            fovy: 70.0,
            znear: 0.01,
            zfar: 100.0,
        }
    }
}
