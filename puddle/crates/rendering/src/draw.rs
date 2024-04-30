use vulkano::{
    command_buffer::{
        sys::CommandBufferBeginInfo, CommandBufferLevel, CommandBufferUsage,
        RecordingCommandBuffer, RenderingAttachmentInfo, RenderingInfo,
    },
    descriptor_set::{DescriptorSet, WriteDescriptorSet},
    pipeline::Pipeline,
    render_pass::{AttachmentLoadOp, AttachmentStoreOp},
    swapchain::{acquire_next_image, SwapchainPresentInfo},
    sync::{self, GpuFuture},
    Validated, VulkanError,
};

use crate::instances::Surface;

pub fn draw(world: &mut legion::World, resources: &mut legion::Resources) {
    let mut renderer = resources
        .get_mut::<super::setup::ForwardRenderer>()
        .unwrap();
    let schene = resources.get::<super::setup::ExampleScene>().unwrap();
    let window = resources.get::<window::PuddleWindow>().unwrap();

    let image_extent: [u32; 2] = window.window.inner_size().into();

    if image_extent.contains(&0) {
        return;
    }

    use legion::IntoQuery;
    let mut query = <&mut Surface>::query();

    for render_surface in query.iter_mut(world) {
        let uniform_buffer_subbuffer = {
            let elapsed = schene.rotation_start.elapsed().as_secs_f32();

            let uniform_data = crate::setup::fs::Data {
                screen_size: image_extent,
                time: elapsed,
            };

            let subbuffer = schene.uniform_buffer.write(uniform_data);

            subbuffer
        };

        let layout = &schene.pipeline.layout().set_layouts()[0];
        let set = DescriptorSet::new(
            schene.descriptor_set_allocator.clone(),
            layout.clone(),
            [WriteDescriptorSet::buffer(0, uniform_buffer_subbuffer)],
            [],
        )
        .unwrap();

        // will keep accumulating and you will eventually reach an out of memory error.
        // Calling this function polls various fences in order to determine what the GPU
        // has already processed, and frees the resources that are no longer needed.
        renderer
            .previous_frame_end
            .as_mut()
            .unwrap()
            .cleanup_finished();

        // Whenever the window resizes we need to recreate everything dependent on the
        // window size. In this example that includes the swapchain, the framebuffers and
        // the dynamic state viewport.
        if render_surface.recreate_swapchain {
            render_surface.reload_swapchain(image_extent);
        }

        // Before we can draw on the output, we have to *acquire* an image from the
        // swapchain. If no image is available (which happens if you submit draw commands
        // too quickly), then the function will block. This operation returns the index of
        // the image that we are allowed to draw upon.
        //
        // This function can block if no image is available. The parameter is an optional
        // timeout after which the function call will return an error.
        let (image_index, suboptimal, acquire_future) =
            match acquire_next_image(render_surface.swapchain.clone(), None)
                .map_err(Validated::unwrap)
            {
                Ok(r) => r,
                Err(VulkanError::OutOfDate) => {
                    render_surface.recreate_swapchain = true;
                    return;
                }
                Err(e) => panic!("failed to acquire next image: {e}"),
            };

        // `acquire_next_image` can be successful, but suboptimal. This means that the
        // swapchain image will still work, but it may not display correctly. With some
        // drivers this can be when the window resizes, but it may not cause the swapchain
        // to become out of date.
        if suboptimal {
            render_surface.recreate_swapchain = true;
        }

        // In order to draw, we have to record a *command buffer*. The command buffer object
        // holds the list of commands that are going to be executed.
        //
        // Recording a command buffer is an expensive operation (usually a few hundred
        // microseconds), but it is known to be a hot path in the driver and is expected to
        // be optimized.
        //
        // Note that we have to pass a queue family when we create the command buffer. The
        // command buffer will only be executable on that given queue family.
        let mut builder = RecordingCommandBuffer::new(
            renderer.command_buffer_allocator.clone(),
            renderer.queue.queue_family_index(),
            CommandBufferLevel::Primary,
            CommandBufferBeginInfo {
                usage: CommandBufferUsage::OneTimeSubmit,
                ..Default::default()
            },
        )
        .unwrap();

        builder
            // Before we can draw, we have to *enter a render pass*. We specify which
            // attachments we are going to use for rendering here, which needs to match
            // what was previously specified when creating the pipeline.
            .begin_rendering(RenderingInfo {
                // As before, we specify one color attachment, but now we specify the image
                // view to use as well as how it should be used.
                color_attachments: vec![Some(RenderingAttachmentInfo {
                    // `Clear` means that we ask the GPU to clear the content of this
                    // attachment at the start of rendering.
                    load_op: AttachmentLoadOp::Clear,
                    // `Store` means that we ask the GPU to store the rendered output in
                    // the attachment image. We could also ask it to discard the result.
                    store_op: AttachmentStoreOp::Store,
                    // The value to clear the attachment with. Here we clear it with a blue
                    // color.
                    //
                    // Only attachments that have `AttachmentLoadOp::Clear` are provided
                    // with clear values, any others should use `None` as the clear value.
                    clear_value: Some([0.0, 0.0, 1.0, 1.0].into()),
                    ..RenderingAttachmentInfo::image_view(
                        // We specify image view corresponding to the currently acquired
                        // swapchain image, to use for this attachment.
                        render_surface.attachment_image_views[image_index as usize].clone(),
                    )
                })],
                ..Default::default()
            })
            .unwrap()
            .set_viewport(0, [render_surface.viewport.clone()].into_iter().collect())
            .unwrap()
            .bind_pipeline_graphics(schene.pipeline.clone())
            .unwrap()
            .bind_descriptor_sets(
                vulkano::pipeline::PipelineBindPoint::Graphics,
                schene.pipeline.layout().clone(),
                0,
                set,
            )
            .unwrap();

        unsafe {
            builder
                // We add a draw command.
                .draw(6, 1, 0, 0)
                .unwrap();
        }

        builder
            // We leave the render pass.
            .end_rendering()
            .unwrap();

        // Finish recording the command buffer by calling `end`.
        let command_buffer = builder.end().unwrap();

        let future = renderer
            .previous_frame_end
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(renderer.queue.clone(), command_buffer)
            .unwrap()
            // The color output is now expected to contain our triangle. But in order to
            // show it on the screen, we have to *present* the image by calling
            // `then_swapchain_present`.
            //
            // This function does not actually present the image immediately. Instead it
            // submits a present command at the end of the queue. This means that it will
            // only be presented once the GPU has finished executing the command buffer
            // that draws the triangle.
            .then_swapchain_present(
                renderer.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(
                    render_surface.swapchain.clone(),
                    image_index,
                ),
            )
            .then_signal_fence_and_flush();

        match future.map_err(Validated::unwrap) {
            Ok(future) => {
                renderer.previous_frame_end = Some(future.boxed());
            }
            Err(VulkanError::OutOfDate) => {
                render_surface.recreate_swapchain = true;
                renderer.previous_frame_end = Some(sync::now(renderer.device.clone()).boxed());
            }
            Err(e) => {
                println!("failed to flush future: {e}");
                renderer.previous_frame_end = Some(sync::now(renderer.device.clone()).boxed());
            }
        }
    }
}
