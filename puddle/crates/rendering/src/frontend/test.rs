use crate::backend::Renderer;

use super::RenderPass;

pub fn init(world: &mut legion::World, resources: &mut legion::Resources) {
    let mut backend = resources.get_mut::<Renderer>().unwrap();

    let mut context = match backend.create_render_context() {
        Some(r) => r,
        None => return,
    };

    context.add_renderpass(RenderPass::ClearColor {
        color: [1.0, 0.0, 0.0, 0.0],
    });

    context.flush(&mut backend);
}
