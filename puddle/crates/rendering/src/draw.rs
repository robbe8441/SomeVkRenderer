#![allow(unused, dead_code)]

use std::sync::Arc;
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, sys::CommandBufferBeginInfo, CommandBufferLevel,
        CommandBufferUsage, RecordingCommandBuffer, RenderingAttachmentInfo, RenderingInfo,
    },
    device::{
        self, physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions,
        QueueCreateInfo, QueueFlags,
    },
    image::{view::ImageView, Image, ImageUsage},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            subpass::PipelineRenderingCreateInfo,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        DynamicState, GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    },
    render_pass::{AttachmentLoadOp, AttachmentStoreOp},
    swapchain::{
        acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
    },
    sync::{self, GpuFuture},
    Validated, Version, VulkanError, VulkanLibrary,
};

    // #[state] recreate_swapchain: &mut bool,
    // #[state] previous_frame_end: &mut Option<Box<dyn GpuFuture>>,
    // #[resource] renderer: &mut super::setup::ForwardRenderer,
    // #[resource] window: &window::PuddleWindow,
    // #[resource] schene: &super::setup::ExampleSchene,

pub fn draw(
    _world: &mut legion::World,
    resources : &mut legion::Resources,
) {

    let mut renderer = resources.get_mut::<super::setup::ForwardRenderer>().unwrap();
    let schene = resources.get::<super::setup::ExampleSchene>().unwrap();
    let window = resources.get::<window::PuddleWindow>().unwrap();

    let image_extent: [u32; 2] = window.window.inner_size().into();

    if image_extent.contains(&0) {
        return;
    }

    // It is important to call this function from time to time, otherwise resources
    // will keep accumulating and you will eventually reach an out of memory error.
    // Calling this function polls various fences in order to determine what the GPU
    // has already processed, and frees the resources that are no longer needed.
    renderer.previous_frame_end.as_mut().unwrap().cleanup_finished();

    // Whenever the window resizes we need to recreate everything dependent on the
    // window size. In this example that includes the swapchain, the framebuffers and
    // the dynamic state viewport.
    if renderer.recreate_swapchain {
        let (new_swapchain, new_images) = renderer.swapchain
            .recreate(SwapchainCreateInfo {
                image_extent,
                ..renderer.swapchain.create_info()
            })
            .expect("failed to recreate swapchain");

        renderer.swapchain = new_swapchain;

        // Now that we have new swapchain images, we must create new image views from
        // them as well.
        renderer.attachment_image_views = window_size_dependent_setup(&new_images, &mut renderer.viewport);

        renderer.recreate_swapchain = false;
    }

    // Before we can draw on the output, we have to *acquire* an image from the
    // swapchain. If no image is available (which happens if you submit draw commands
    // too quickly), then the function will block. This operation returns the index of
    // the image that we are allowed to draw upon.
    //
    // This function can block if no image is available. The parameter is an optional
    // timeout after which the function call will return an error.
    let (image_index, suboptimal, acquire_future) =
        match acquire_next_image(renderer.swapchain.clone(), None).map_err(Validated::unwrap) {
            Ok(r) => r,
            Err(VulkanError::OutOfDate) => {
                renderer.recreate_swapchain = true;
                return;
            }
            Err(e) => panic!("failed to acquire next image: {e}"),
        };

    // `acquire_next_image` can be successful, but suboptimal. This means that the
    // swapchain image will still work, but it may not display correctly. With some
    // drivers this can be when the window resizes, but it may not cause the swapchain
    // to become out of date.
    if suboptimal {
        renderer.recreate_swapchain = true;
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
                    renderer.attachment_image_views[image_index as usize].clone(),
                )
            })],
            ..Default::default()
        })
        .unwrap()
        // We are now inside the first subpass of the render pass.
        //
        // TODO: Document state setting and how it affects subsequent draw commands.
        .set_viewport(0, [renderer.viewport.clone()].into_iter().collect())
        .unwrap()
        .bind_pipeline_graphics(schene.pipeline.clone())
        .unwrap()
        .bind_vertex_buffers(0, schene.vertex_buffer.clone())
        .unwrap();

    unsafe {
        builder
            // We add a draw command.
            .draw(schene.vertex_buffer.len() as u32, 1, 0, 0)
            .unwrap();
    }

    builder
        // We leave the render pass.
        .end_rendering()
        .unwrap();

    // Finish recording the command buffer by calling `end`.
    let command_buffer = builder.end().unwrap();

    let future = renderer.previous_frame_end
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
            SwapchainPresentInfo::swapchain_image_index(renderer.swapchain.clone(), image_index),
        )
        .then_signal_fence_and_flush();

    match future.map_err(Validated::unwrap) {
        Ok(future) => {
            renderer.previous_frame_end = Some(future.boxed());
        }
        Err(VulkanError::OutOfDate) => {
            renderer.recreate_swapchain = true;
            renderer.previous_frame_end = Some(sync::now(renderer.device.clone()).boxed());
        }
        Err(e) => {
            println!("failed to flush future: {e}");
            renderer.previous_frame_end = Some(sync::now(renderer.device.clone()).boxed());
        }
    }
}

fn window_size_dependent_setup(
    images: &[Arc<Image>],
    viewport: &mut Viewport,
) -> Vec<Arc<ImageView>> {
    let extent = images[0].extent();
    viewport.extent = [extent[0] as f32, extent[1] as f32];

    images
        .iter()
        .map(|image| ImageView::new_default(image.clone()).unwrap())
        .collect::<Vec<_>>()
}
