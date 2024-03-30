#![allow(unused, dead_code)]

mod backend;
mod frontend;
mod types;

pub struct RenderPlugin;
pub use types::Vertex;
pub use wgpu::{self, RenderPipeline};

pub use backend::{
    BindGroupLayout, BindGroupLayoutEntry, BindingType, Buffer, CullMode, RenderPipelineDesc,
    Renderer,
};
pub use frontend::RenderPass;

impl application::Plugin for RenderPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        let backend = backend::Renderer::new(app);

        app.resources.insert(backend);
    }
}
