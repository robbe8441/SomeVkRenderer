use vulkano::{pipeline::PipelineShaderStageCreateInfo, shader::EntryPoint};

pub struct GraphicsShaderSet {
    pub vertex: EntryPoint,
    pub fragment: Option<EntryPoint>,
}

impl GraphicsShaderSet {
    pub fn default(device: &crate::Device) -> Self {
        let vs = vs::load(device.device.clone())
            .unwrap()
            .entry_point("main")
            .unwrap();
        let fs = fs::load(device.device.clone())
            .unwrap()
            .entry_point("main")
            .unwrap();

        Self {
            vertex: vs,
            fragment: Some(fs),
        }
    }

    pub fn to_vec(&self) -> Vec<EntryPoint> {
        let mut vec = vec![self.vertex.clone()];

        if let Some(frag) = &self.fragment {
            vec.push(frag.clone())
        }
        vec
    }

    pub fn to_desc(&self) -> Vec<PipelineShaderStageCreateInfo> {
        let vec = self.to_vec();

        vec.into_iter()
            .map(|e| PipelineShaderStageCreateInfo::new(e))
            .collect()
    }
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
                #version 450

                layout(location = 0) in vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: r"
                #version 450

                layout(location = 0) out vec4 f_color;

                void main() {
                    f_color = vec4(1.0, 0.0, 0.0, 1.0);
                }
            ",
    }
}
