#![allow(unused, dead_code)]

mod backend;
mod frontend;
mod types;

pub struct RenderPlugin;
pub use types::Vertex;
pub use wgpu::{RenderPipeline, self};

pub use backend::{Buffer, Renderer, RenderPipelineDesc};
pub use frontend::RenderPass;

impl application::Plugin for RenderPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        let backend = backend::Renderer::new(app);

        app.resources.insert(backend);
    }
}
