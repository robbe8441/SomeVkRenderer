use super::*;

pub fn init(app: &mut application::Application) {
    use application::Schedules;
    app.schedules
        .add_non_parallel(Schedules::Update, |world, resources| {
            let mut renderer = match resources.get_mut::<Renderer>() {
                Some(r) => r,
                None => {
                    warn!("cant find renderer");
                    return;
                }
            };
            let mut camera = match resources.get::<utils::Camera>() {
                Some(r) => r,
                None => {
                    warn!("cant find renderer");
                    return;
                }
            };

            let mut context = match renderer.create_render_context() {
                Some(r) => r,
                None => return,
            };

            context.add_renderpass(RenderPass::ClearColor {
                color: [0.0, 0.0, 0.01, 1.0],
            });

            use application::legion::query::*;
            let mut models = <&utils::MeshAsset>::query();

            for model in models.iter(world) {
                let mut bind_groups = vec![&camera.bind_group];
                bind_groups.extend(model.material.bind_groups.iter());

                context.add_renderpass(RenderPass::DrawIndexed {
                    vertex_buffer: &model.vertex_buffer,
                    index_buffer: &model.index_buffer,
                    //instance_buffer: &model.instance_buffer,
                    //instance_range: 0..model.instance_buffer.length.min(1) as u32,
                    pipeline: &model.material.pipeline,
                    bind_groups: &bind_groups,
                });
            }

            context.flush(&mut renderer);
        });

    let renderer = app.resources
        .get::<Renderer>()
        .expect("failed to crate camera: renderer does not exist");

    let cam = utils::Camera::new(&renderer);

    drop(renderer);

    app.resources.insert(cam);
}
