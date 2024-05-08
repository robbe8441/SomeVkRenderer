pub mod backend;
use application::{Startup, Update};
use backend::{device::RenderDevice, swapchain::Swapchain};
use bevy_ecs::system::{Commands, NonSendMut, Res, ResMut, Resource};
pub use vulkano;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};
use vulkano::sync::{self, GpuFuture};
use vulkano::{
    buffer::BufferContents,
    image::view::ImageView,
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        PipelineLayout, PipelineShaderStageCreateInfo,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo},
    swapchain::{acquire_next_image, SwapchainPresentInfo},
    Validated, VulkanError,
};

pub struct RenderPlugin;

impl application::Plugin for RenderPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        backend::init(app);

        app.add_systems(Update, setup_pipeline);

        app.add_systems(Startup, create_vertex_buffer);

        app.world.insert_non_send_resource(PreviousFrameEnd(None));
    }
}

pub struct PreviousFrameEnd(Option<Box<dyn GpuFuture>>);

#[derive(Resource)]
struct VertexBuffer(Subbuffer<[MyVertex]>);

fn create_vertex_buffer(
    memory_allocator: Res<backend::buffer::StandardMemoryAllocator>,
    mut commands: Commands,
    ) {
    let vertex_buffer = Buffer::from_iter(
        memory_allocator.0.clone(),
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        vec![
            MyVertex {
                position: [0.0, 0.0],
            },
            MyVertex {
                position: [1.0, 0.0],
            },
            MyVertex {
                position: [1.0, 1.0],
            },
        ],
    )
    .unwrap();

        commands.insert_resource(VertexBuffer(vertex_buffer));
}





fn setup_pipeline(
    device: Res<RenderDevice>,
    mut swapchain: ResMut<Swapchain>,
    mut previous_frame_end: NonSendMut<PreviousFrameEnd>,
    command_buffer_allocator: Res<backend::buffer::CommandBufferAllocator>,
    vertex_buffer: Res<VertexBuffer>,
) {
    previous_frame_end.0.as_mut().unwrap().cleanup_finished();

    let vs = vs::load(device.device.clone()).expect("failed to create shader module");
    let fs = fs::load(device.device.clone()).expect("failed to create shader module");

    use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
    use vulkano::pipeline::graphics::vertex_input::Vertex;
    use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
    use vulkano::pipeline::GraphicsPipeline;
    use vulkano::render_pass::Subpass;


    let render_pass = vulkano::single_pass_renderpass!(
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
    .unwrap();

    let framebuffers = swapchain
        .images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>();

    use vulkano::command_buffer::{
        auto::RecordingCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents,
        SubpassEndInfo,
    };

    let mut builder = RecordingCommandBuffer::new(
        command_buffer_allocator.0.clone(),
        device.queue.queue_family_index(),
        vulkano::command_buffer::CommandBufferLevel::Primary,
        vulkano::command_buffer::CommandBufferBeginInfo {
            usage: vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
            ..Default::default()
        },
    )
    .unwrap();

    let vs = vs.entry_point("main").unwrap();
    let fs = fs.entry_point("main").unwrap();

    let vertex_input_state = MyVertex::per_vertex().definition(&vs).unwrap();

    let stages = [
        PipelineShaderStageCreateInfo::new(vs),
        PipelineShaderStageCreateInfo::new(fs),
    ];

    let layout = PipelineLayout::new(
        device.device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
            .into_pipeline_layout_create_info(device.device.clone())
            .unwrap(),
    )
    .unwrap();

    let (image_index, suboptimal, acquire_future) =
        match acquire_next_image(swapchain.swapchain.clone(), None).map_err(Validated::unwrap) {
            Ok(r) => r,
            Err(VulkanError::OutOfDate) => {
                swapchain.recreate_swapchain = true;
                return;
            }
            Err(e) => panic!("failed to acquire next image: {e}"),
        };

    if suboptimal {
        swapchain.recreate_swapchain = true;
    }

    let subpass = Subpass::from(render_pass.clone(), 0).unwrap();

    let extent = swapchain.swapchain.image_extent();

    let pipeline = GraphicsPipeline::new(
        device.device.clone(),
        None,
        GraphicsPipelineCreateInfo {
            stages: stages.into_iter().collect(),
            vertex_input_state: Some(vertex_input_state),
            input_assembly_state: Some(InputAssemblyState::default()),
            viewport_state: Some(ViewportState {
                viewports: [Viewport {
                    offset: [0.0, 0.0],
                    extent: [extent[0] as f32, extent[1] as f32],
                    depth_range: 0.0..=1.0,
                }]
                .into_iter()
                .collect(),
                ..Default::default()
            }),
            rasterization_state: Some(RasterizationState::default()),
            multisample_state: Some(MultisampleState::default()),
            color_blend_state: Some(ColorBlendState::with_attachment_states(
                subpass.num_color_attachments(),
                ColorBlendAttachmentState::default(),
            )),
            subpass: Some(subpass.into()),
            ..GraphicsPipelineCreateInfo::layout(layout)
        },
    )
    .unwrap();

    builder
        .begin_render_pass(
            RenderPassBeginInfo {
                clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
                ..RenderPassBeginInfo::framebuffer(framebuffers[image_index as usize].clone())
            },
            SubpassBeginInfo {
                contents: SubpassContents::Inline,
                ..Default::default()
            },
        )
        .unwrap()
        .bind_pipeline_graphics(pipeline.clone())
        .unwrap()
        .bind_vertex_buffers(0, vertex_buffer.0.clone())
        .unwrap();

    unsafe { builder.draw(3, 1, 0, 0) }
        .unwrap()
        .end_render_pass(SubpassEndInfo::default())
        .unwrap();

    let command_buffer = builder.end().unwrap();

    let future = previous_frame_end
        .0
        .take()
        .unwrap()
        .join(acquire_future)
        .then_execute(device.queue.clone(), command_buffer)
        .unwrap()
        .then_swapchain_present(
            device.queue.clone(),
            SwapchainPresentInfo::swapchain_image_index(swapchain.swapchain.clone(), image_index),
        )
        .then_signal_fence_and_flush();

    match future.map_err(Validated::unwrap) {
        Ok(future) => {
            previous_frame_end.0 = Some(future.boxed());
        }
        Err(VulkanError::OutOfDate) => {
            swapchain.recreate_swapchain = true;
            previous_frame_end.0 = Some(sync::now(device.device.clone()).boxed());
        }
        Err(e) => {
            println!("failed to flush future: {e}");
            previous_frame_end.0 = Some(sync::now(device.device.clone()).boxed());
        }
    }
}

#[repr(C)]
#[derive(BufferContents, Vertex)]
struct MyVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 460

            layout(location = 0) in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        ",
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
            #version 460

            layout(location = 0) out vec4 f_color;

            void main() {
                f_color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        ",
    }
}
