use bevy_ecs::system::{NonSend, NonSendMut, Query, Res, ResMut};
use vulkano::{
    descriptor_set::{DescriptorSet, WriteDescriptorSet},
    image::view::ImageView,
    pipeline::PipelineBindPoint,
    render_pass::{Framebuffer, FramebufferCreateInfo},
    swapchain::{acquire_next_image, SwapchainPresentInfo},
    sync::GpuFuture,
    Validated, VulkanError,
};

use crate::backend::{
    self,
    device::{PreviousFrameEnd, RenderDevice},
};

use super::{
    setup::PipelineSetup,
    types::{Camera, VertexBuffer},
};

pub fn draw(
    device: Res<RenderDevice>,
    mut swapchain: ResMut<backend::swapchain::Swapchain>,
    mut previous_frame_end: NonSendMut<PreviousFrameEnd>,
    command_buffer_allocator: Res<backend::buffer::CommandBufferAllocator>,
    descriptor_set_allocator: Res<backend::buffer::DescriptorSetAllocator>,
    vertex_buffer_query: Query<&VertexBuffer>,
    pipeline_setup: Res<PipelineSetup>,
    camera: NonSend<Camera>,
) {
    previous_frame_end.0.as_mut().unwrap().cleanup_finished();

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

    let framebuffer = {
        let image = swapchain.images[image_index as usize].clone();

        let view = ImageView::new_default(image.clone()).unwrap();
        Framebuffer::new(
            pipeline_setup.render_pass.clone(),
            FramebufferCreateInfo {
                attachments: vec![view],
                ..Default::default()
            },
        )
        .unwrap()
    };

    builder
        .begin_render_pass(
            RenderPassBeginInfo {
                clear_values: vec![Some([0.0, 0.0, 0.0, 1.0].into())],
                ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
            },
            SubpassBeginInfo {
                contents: SubpassContents::Inline,
                ..Default::default()
            },
        )
        .unwrap()
        .bind_pipeline_graphics(pipeline_setup.pipeline.clone())
        .unwrap();

    let cam_uniform = camera.build_proj();

    use vulkano::pipeline::Pipeline;
    let layout = &pipeline_setup.pipeline.layout().set_layouts()[0];
    let set = DescriptorSet::new(
        descriptor_set_allocator.0.clone(),
        layout.clone(),
        [WriteDescriptorSet::buffer(0, cam_uniform)],
        [],
    )
    .unwrap();

    builder
        .bind_descriptor_sets(
            PipelineBindPoint::Graphics,
            pipeline_setup.pipeline.layout().clone(),
            0,
            set,
        )
        .unwrap();

    for vertex_buffer in vertex_buffer_query.iter() {
        builder
            .bind_vertex_buffers(0, vertex_buffer.0.clone())
            .unwrap();

        unsafe { builder.draw(vertex_buffer.0.len() as u32, 1, 0, 0) }.unwrap();
    }
    builder.end_render_pass(SubpassEndInfo::default()).unwrap();

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
            previous_frame_end.0 = Some(vulkano::sync::now(device.device.clone()).boxed());
        }
        Err(e) => {
            println!("failed to flush future: {e}");
            previous_frame_end.0 = Some(vulkano::sync::now(device.device.clone()).boxed());
        }
    }
}
