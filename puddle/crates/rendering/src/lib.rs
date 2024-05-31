pub struct RenderPlugin;

impl application::Plugin for RenderPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        app.add_systems(application::PreStartup, vulkan::init);
        app.add_systems(application::PostUpdate, vulkan::recreate_swapchain_system);

        app.add_systems(application::Startup, setup_render_stuff);

        app.add_systems(application::Update, draw);
    }
}

use core::panic;

use bevy_ecs::system::{Commands, NonSendMut, Res, ResMut, Resource};
use vulkan::*;

#[derive(Resource)]
struct RenderSetup {
    pipeline: graphics::GraphicsPipeline,
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
}

fn setup_render_stuff(
    device: Res<Device>,
    swapchain: Res<Swapchain>,
    memory_allocator: Res<StandardMemoryAllocator>,
    mut commands: Commands,
) {
    let shaders = shaders::GraphicsShaderSet::default(&device);

    let pipeline = graphics::GraphicsPipeline::new(
        &device,
        &graphics::GraphicsPipelineDescriber {
            shaders,
            extent: swapchain.swapchain.image_extent(),
            render_pass: vulkan::single_pass_renderpass!(
            device.device.clone(),
            attachments: {
                color: {
                    format: swapchain.swapchain.image_format(),
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
            },
            pass: {
                color: [color],
                depth_stencil: {},
            },
            )
            .unwrap(),
        },
    );

    let mesh = utils::Mesh::plane();

    let vertex_buffer = VertexBuffer::new(&memory_allocator, mesh.vertecies);
    let index_buffer = IndexBuffer::new(&memory_allocator, mesh.indices);

    commands.insert_resource(RenderSetup {
        pipeline,
        vertex_buffer,
        index_buffer,
    })
}

fn draw(
    render_setup: Res<RenderSetup>,
    device: Res<Device>,
    mut swapchain: ResMut<Swapchain>,
    command_buffer_allocator: Res<CommandBufferAllocator>,
    mut render_context: NonSendMut<RenderContext>,
) {
    render_context.begin_render(&device, &mut swapchain, &command_buffer_allocator);

    let framebuffer = {
        let image = swapchain.images[render_context.image_index.unwrap() as usize].clone();

        let view = ImageView::new_default(image.clone()).unwrap();
        vulkan::Framebuffer::new(
            match render_setup.pipeline.0.subpass() {
                PipelineSubpassType::BeginRenderPass(r) => r.render_pass().clone(),
                _ => panic!("something went wrong"),
            },
            vulkan::FramebufferCreateInfo {
                attachments: vec![view],
                ..Default::default()
            },
        )
        .unwrap()
    };

    render_context
        .begin_render_pass(RenderPassBeginInfo {
            clear_values: vec![Some([0.0, 0.0, 0.0, 1.0].into())],
            ..vulkan::RenderPassBeginInfo::framebuffer(framebuffer.clone())
        })
        .bind_pipeline_graphics(&render_setup.pipeline)
        .bind_vertex_buffers(0, render_setup.vertex_buffer.0.clone())
        .bind_index_buffer(render_setup.index_buffer.0.clone())
        .draw_indexed(render_setup.index_buffer.0.len() as u32, 1, 0, 0, 0)
        .end_render_pass();

    render_context.submit(&device, &mut swapchain);
}
