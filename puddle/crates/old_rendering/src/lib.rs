#![allow(unused, dead_code)]

mod backend;
mod frontend;
mod setup;
pub mod utils;

pub struct RenderPlugin;
use application::log::warn;
pub use backend::{
    BindGroupLayout, BindGroupLayoutEntry, BindingType, Buffer, CullMode, RenderPipelineDesc,
    Renderer,
};
pub use frontend::RenderPass;
pub use utils::Vertex;
pub use wgpu::{self, RenderPipeline};

impl application::Plugin for RenderPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        let backend = backend::Renderer::new(app);
        app.resources.insert(backend);

        setup::init(app);
    }
}
