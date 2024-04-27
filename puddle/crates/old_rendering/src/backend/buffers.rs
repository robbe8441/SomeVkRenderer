use std::sync::Arc;

#[derive(Clone)]
pub struct Buffer {
    pub buffer: Arc<wgpu::Buffer>,
    pub length : usize
}

impl Buffer {

    #[inline(always)]
    pub fn binding(&self) -> wgpu::BindingResource<'_> {
        self.buffer.as_entire_binding()
    }
}


