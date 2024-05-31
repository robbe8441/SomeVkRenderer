use std::sync::Arc;

use vulkano::{
    command_buffer::RecordingCommandBuffer, swapchain::SwapchainPresentInfo, sync::GpuFuture,
    Validated, VulkanError,
};

// a struct representing the FUTURE!!!
// TODO : edit comment
pub struct PreviousFrameEnd(pub Option<Box<dyn vulkano::sync::GpuFuture>>);

impl PreviousFrameEnd {
    pub fn new(device: &crate::Device) -> Self {
        Self(Some(vulkano::sync::now(device.device.clone()).boxed()))
    }

    #[inline(always)]
    pub fn submit_buffer(
        &mut self,
        buffer: RecordingCommandBuffer,
        queue: Arc<vulkano::device::Queue>,
    ) -> &mut Self {
        let future = self
            .0
            .take()
            .unwrap()
            .then_execute(queue, buffer.end().unwrap())
            .unwrap();

        self.0 = Some(future.boxed());

        self
    }

    // clean up finished submissions
    #[inline(always)]
    pub fn cleanup_finished(&mut self) {
        self.0.as_mut().unwrap().cleanup_finished();
    }

    pub fn submit_render_context(
        &mut self,
        device: &crate::Device,
        render_context: &mut crate::RenderContext,
        swapchain: &mut crate::Swapchain,
    ) {
        let future = self
            .0
            .take()
            .unwrap()
            .join(render_context.acquire_future.take().unwrap())
            .then_execute(
                device.render_queue().clone(),
                render_context
                    .recording_buffer
                    .take()
                    .unwrap()
                    .end()
                    .unwrap(),
            )
            .unwrap()
            .then_swapchain_present(
                device.render_queue().clone(),
                SwapchainPresentInfo::swapchain_image_index(
                    swapchain.swapchain.clone(),
                    render_context.image_index.unwrap(),
                ),
            )
            .then_signal_fence_and_flush();

        match future.map_err(Validated::unwrap) {
            Ok(future) => {
                self.0 = Some(future.boxed());
            }
            Err(VulkanError::OutOfDate) => {
                swapchain.recreate_swapchain = true;
                self.0 = Some(vulkano::sync::now(device.device.clone()).boxed());
            }
            Err(e) => {
                println!("failed to flush future: {e}");
                self.0 = Some(vulkano::sync::now(device.device.clone()).boxed());
            }
        };
    }
}
