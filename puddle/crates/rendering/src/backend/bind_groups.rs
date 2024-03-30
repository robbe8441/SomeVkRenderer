use super::Renderer;

pub struct BindGroupLayout<'a> {
    pub entries: Vec<BindGroupLayoutEntry<'a>>,
}

pub struct BindGroupLayoutEntry<'a> {
    pub visibility: wgpu::ShaderStages,
    pub ty: BindingType,
    pub resource: wgpu::BindingResource<'a>,
}

#[derive(Clone)]
pub enum BindingType {
    UIntTexture(wgpu::TextureViewDimension),

    SIntTexture(wgpu::TextureViewDimension),

    Sampler(wgpu::SamplerBindingType),

    StorrageTexture {
        access: wgpu::StorageTextureAccess,
        format: wgpu::TextureFormat,
        dimension: wgpu::TextureViewDimension,
    },

    Buffer(wgpu::BufferBindingType),
}

impl Into<wgpu::BindingType> for BindingType {
    fn into(self) -> wgpu::BindingType {
        match self {
            BindingType::SIntTexture(dimension) => wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Sint,
                view_dimension: dimension,
                multisampled: false,
            },
            BindingType::UIntTexture(dimension) => wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Uint,
                view_dimension: dimension,
                multisampled: false,
            },
            BindingType::StorrageTexture {
                access,
                format,
                dimension,
            } => wgpu::BindingType::StorageTexture {
                access,
                format,
                view_dimension: dimension,
            },
            BindingType::Sampler(layout) => wgpu::BindingType::Sampler(layout),
            BindingType::Buffer(ty) => wgpu::BindingType::Buffer {
                ty,
                min_binding_size: None,
                has_dynamic_offset: false,
            },
        }
    }
}

impl<'a> BindGroupLayout<'a> {
    #[inline]
    fn push(&mut self, desc: BindGroupLayoutEntry<'a>) {
        self.entries.push(desc);
    }

    pub fn build(self, renderer: &super::Renderer) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
        let layout_entries: Vec<wgpu::BindGroupLayoutEntry> = self
            .entries
            .iter()
            .enumerate()
            .map(|(i, v)| wgpu::BindGroupLayoutEntry {
                binding: i as u32,
                visibility: v.visibility,
                ty: v.ty.clone().into(),
                count: None,
            })
            .collect();

        let layout = renderer
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &layout_entries,
                label: Some("bind group layout"),
            });

        let entries: Vec<wgpu::BindGroupEntry> = self
            .entries
            .into_iter()
            .enumerate()
            .map(|(i, v)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: v.resource,
            })
            .collect();

        let bindgroup = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("bind group"),
                layout: &layout,
                entries: &entries,
            });

        (bindgroup, layout)
    }
}
