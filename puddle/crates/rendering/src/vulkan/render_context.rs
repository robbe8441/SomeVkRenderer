use vulkano::{
    command_buffer::{
        RecordingCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents,
    },
    pipeline::graphics::vertex_input::VertexBuffersCollection,
    swapchain::{acquire_next_image, SwapchainAcquireFuture, SwapchainPresentInfo},
    sync::GpuFuture,
    Validated, VulkanError,
};

use crate::{Device, Swapchain};

pub struct RenderContext {
    pub image_index: Option<u32>,
    gpu_future: Option<Box<dyn GpuFuture>>,
    recording_buffer: Option<RecordingCommandBuffer>,
    acquire_future: Option<SwapchainAcquireFuture>,
}

impl RenderContext {
    pub fn new(device: &Device) -> Self {
        Self {
            gpu_future: Some(vulkano::sync::now(device.device.clone()).boxed()),
            recording_buffer: None,
            image_index: None,
            acquire_future: None,
        }
    }

    #[inline(always)]
    pub fn begin_render(
        &mut self,
        device: &Device,
        swapchain: &mut Swapchain,
        command_buffer_allocator: &super::CommandBufferAllocator,
    ) {

        self.gpu_future.as_mut().unwrap().cleanup_finished();

        let builder = RecordingCommandBuffer::new(
            command_buffer_allocator.0.clone(),
            device.render_queue().queue_family_index(),
            vulkano::command_buffer::CommandBufferLevel::Primary,
            vulkano::command_buffer::CommandBufferBeginInfo {
                usage: vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
                ..Default::default()
            },
        )
        .unwrap();


        let (image_index, suboptimal, acquire_future) = match acquire_next_image(
            swapchain.swapchain.clone(),
            None,
        )
        .map_err(Validated::unwrap)
        {
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


        self.recording_buffer = Some(builder);
        self.image_index = Some(image_index);
        self.acquire_future = Some(acquire_future);
    }

    #[inline(always)]
    pub fn begin_render_pass(&mut self, info: RenderPassBeginInfo) -> &mut Self {
        self.recording_buffer
            .as_mut()
            .unwrap()
            .begin_render_pass(
                info,
                SubpassBeginInfo {
                    contents: SubpassContents::Inline,
                    ..Default::default()
                },
            )
            .unwrap();
        self
    }

    #[inline(always)]
    pub fn end_render_pass(&mut self) -> &mut Self {
        self.recording_buffer
            .as_mut()
            .unwrap()
            .end_render_pass(vulkano::command_buffer::SubpassEndInfo::default())
            .unwrap();
        self
    }

    #[inline(always)]
    pub fn bind_pipeline_graphics(
        &mut self,
        pipeline: &crate::vulkan::pipeline::graphics::GraphicsPipeline,
    ) -> &mut Self {
        self.recording_buffer
            .as_mut()
            .unwrap()
            .bind_pipeline_graphics(pipeline.0.clone())
            .unwrap();
        self
    }

    #[inline(always)]
    pub fn bind_vertex_buffers(
        &mut self,
        first_binding: u32,
        vertex_buffers: impl VertexBuffersCollection,
    ) -> &mut Self {
        self.recording_buffer
            .as_mut()
            .unwrap()
            .bind_vertex_buffers(first_binding, vertex_buffers)
            .unwrap();
        self
    }

    #[inline(always)]
    pub fn draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) -> &mut Self {
        unsafe {
            self.recording_buffer.as_mut().unwrap().draw(
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
        }
        .unwrap();

        self
    }

    #[inline(always)]
    pub fn submit(&mut self, device: &Device, swapchain: &mut Swapchain) {
        if let (Some(command_buffer), Some(gpu_future), Some(acquire_future), Some(image_index)) = (
            self.recording_buffer.take().map(|b| b.end().unwrap()),
            self.gpu_future.take(),
            self.acquire_future.take(),
            self.image_index,
        ) {

            let future = gpu_future
                .join(acquire_future)
                .then_execute(device.render_queue().clone(), command_buffer)
                .unwrap()
                .then_swapchain_present(
                    device.render_queue().clone(),
                    SwapchainPresentInfo::swapchain_image_index(
                        swapchain.swapchain.clone(),
                        image_index,
                    ),
                )
                .then_signal_fence_and_flush();

            match future.map_err(Validated::unwrap) {
                Ok(future) => {
                    self.gpu_future = Some(future.boxed());
                }
                Err(VulkanError::OutOfDate) => {
                    swapchain.recreate_swapchain = true;
                    self.gpu_future = Some(vulkano::sync::now(device.device.clone()).boxed());
                }
                Err(e) => {
                    println!("failed to flush future: {e}");
                    self.gpu_future = Some(vulkano::sync::now(device.device.clone()).boxed());
                }
            };
        }
    }
}
