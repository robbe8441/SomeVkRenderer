use bevy_ecs::system::Resource;
use vulkano::{
    command_buffer::{
        RecordingCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents,
    },
    pipeline::graphics::vertex_input::VertexBuffersCollection,
    swapchain::{acquire_next_image, SwapchainAcquireFuture},
    Validated, VulkanError,
};

use crate::{Device, Swapchain};

pub struct RenderContext {
    pub image_index: Option<u32>,
    pub recording_buffer: Option<RecordingCommandBuffer>,
    pub acquire_future: Option<SwapchainAcquireFuture>,
}

pub enum RunContextError {
    // the swapchain is out of date and needs to be recreated before you render
    SwapchainOutdated,
}

impl RenderContext {
    pub fn new() -> Self {
        Self {
            recording_buffer: None,
            image_index: None,
            acquire_future: None,
        }
    }

    // function that needs to be called before every render to setup the command_buffer
    // and get an image to render on to from the swapchain
    #[inline(always)]
    pub fn begin_render(
        &mut self,
        device: &Device,
        swapchain: &mut Swapchain,
        command_buffer_allocator: &super::CommandBufferAllocator,
    ) -> Result<(), RunContextError> {

        // create a new frame buffer
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
                return Err(RunContextError::SwapchainOutdated);
            }
            Err(e) => panic!("failed to acquire next image: {e}"),
        };

        if suboptimal {
            swapchain.recreate_swapchain = true;
        }

        self.recording_buffer = Some(builder);
        self.image_index = Some(image_index);
        self.acquire_future = Some(acquire_future);

        Ok(())
    }

    // begin a render_pass by setting the framebuffer and render_pass
    // needs to be called before you can record draw() commands
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

    // end the render_pass you started with begin_render_pass
    // needs to be called for every render_pass
    #[inline(always)]
    pub fn end_render_pass(&mut self) -> &mut Self {
        self.recording_buffer
            .as_mut()
            .unwrap()
            .end_render_pass(vulkano::command_buffer::SubpassEndInfo::default())
            .unwrap();
        self
    }

    // set the render_pipeline
    #[inline(always)]
    pub fn bind_pipeline(
        &mut self,
        pipeline: &crate::pipeline::graphics::GraphicsPipeline,
    ) -> &mut Self {
        self.recording_buffer
            .as_mut()
            .unwrap()
            .bind_pipeline_graphics(pipeline.0.clone())
            .unwrap();
        self
    }

    // bind vertex buffers
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

    // bind a index buffer
    // this only works with one buffer for every draw call, unlike vertex buffers
    #[inline(always)]
    pub fn bind_index_buffer(&mut self, buffer: &crate::IndexBuffer) -> &mut Self {
        self.recording_buffer
            .as_mut()
            .unwrap()
            .bind_index_buffer(buffer.0.clone())
            .unwrap();
        self
    }

    // draw using the index buffer and vertex_buffer
    #[inline(always)]
    pub fn draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) -> &mut Self {
        unsafe {
            self.recording_buffer.as_mut().unwrap().draw_indexed(
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            )
        }
        .unwrap();

        self
    }

    // draw using just a vertex_buffer
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
}
