#![allow(unused, dead_code)]
mod wgpu_backend;
pub use wgpu_backend::WebGpu;

pub trait RenderBackend {}

pub struct RenderPlugin<T: RenderBackend>(pub T);

use std::sync::Arc;


pub struct Renderer {
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
}


trait Hehe {}
trait HeheHa {}

struct Muu {
    x : Box<dyn Hehe>,
    y : Box<dyn HeheHa>,
}


