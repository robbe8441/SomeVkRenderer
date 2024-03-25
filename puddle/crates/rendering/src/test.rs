use crate::{BufferType, RenderContext, Renderer};

#[legion::system]
fn setup(
    #[resource] renderer : &mut Box<dyn Renderer>
    ) {

    let buffer = renderer.create_buffer(&BufferType::Uniform(vec![1,2,3,4]));
}
