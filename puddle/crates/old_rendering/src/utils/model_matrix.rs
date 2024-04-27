use crate::utils::InstanceRaw;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ModelMatrix {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

impl ModelMatrix {
    pub fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0].into(),
            rotation: cgmath::Quaternion::new(0.0, 0.0, 0.0, 0.0),
        }
    }
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
            .into(),
        }
    }
}
