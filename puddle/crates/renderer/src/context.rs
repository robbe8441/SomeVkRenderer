use std::sync::Arc;
use wgpu::{CommandBuffer, CommandEncoder};

enum QueuedCommandBuffer<'w> {
    Ready(CommandBuffer),
    Task(Box<dyn FnOnce(&wgpu::Device) -> CommandBuffer + 'w + Send>),
}

pub struct RenderContext<'w> {
    render_device: Arc<wgpu::Device>,
    command_encoder: Option<wgpu::CommandEncoder>,
    command_buffer_queue: Vec<QueuedCommandBuffer<'w>>,
}

impl<'w> RenderContext<'w> {
    pub fn new(device: Arc<wgpu::Device>) -> Self {
        Self {
            render_device: device,
            command_encoder: None,
            command_buffer_queue: Vec::new(),
        }
    }

    pub fn command_encoder(&mut self) -> &mut CommandEncoder {
        self.command_encoder.get_or_insert_with(|| {
            self.render_device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor::default())
        })
    }

    pub fn begin_render_pass<'a>(
        &'a mut self,
        descriptor: wgpu::RenderPassDescriptor<'a, '_>,
    ) -> wgpu::RenderPass {
        println!("{:?}", descriptor.label);
        // Cannot use command_encoder() as we need to split the borrow on self
        let command_encoder = self.command_encoder.get_or_insert_with(|| {
            self.render_device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor::default())
        });
        command_encoder.begin_render_pass(&descriptor)
    }

    pub fn add_command_buffer_generation_task(
        &mut self,
        task: impl FnOnce(&wgpu::Device) -> CommandBuffer + 'w + Send,
    ) {
        self.flush_encoder();
        self.command_buffer_queue
            .push(QueuedCommandBuffer::Task(Box::new(task)));
    }

    pub fn finish(mut self) -> Vec<CommandBuffer> {
        self.flush_encoder();

        let mut command_buffers = Vec::with_capacity(self.command_buffer_queue.len());
        for (i, queued_command_buffer) in self.command_buffer_queue.into_iter().enumerate() {
            match queued_command_buffer {
                QueuedCommandBuffer::Ready(command_buffer) => {
                    command_buffers.push((i, command_buffer));
                }
                QueuedCommandBuffer::Task(command_buffer_generation_task) => {
                    command_buffers.push((
                        i,
                        command_buffer_generation_task(&self.render_device.clone()),
                    ));
                }
            }
        }
        command_buffers.sort_unstable_by_key(|(i, _)| *i);
        command_buffers.into_iter().map(|(_, cb)| cb).collect()
    }

    fn flush_encoder(&mut self) {
        if let Some(encoder) = self.command_encoder.take() {
            self.command_buffer_queue
                .push(QueuedCommandBuffer::Ready(encoder.finish()));
        }
    }
}
