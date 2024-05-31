use std::sync::Arc;

use bevy_ecs::{component::Component, system::{NonSendMut, ResMut, Resource}};
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::{
        CommandBufferBeginInfo, CommandBufferLevel, CommandBufferUsage, CopyBufferToImageInfo,
        RecordingCommandBuffer,
    },
    image::{view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
};

use crate::gpu_future::PreviousFrameEnd;

#[derive(Component, Resource)]
pub struct TextureView(Arc<ImageView>);

pub struct TextureDescriber {
    format: vulkano::format::Format,
    image_type: ImageType,
    extent: [u32; 3],
    array_layers: u32,
    bytes: Vec<u8>,
}

impl TextureView {
    pub fn new(
        command_buffer_allocator: &crate::CommandBufferAllocator,
        memory_allocator: &crate::StandardMemoryAllocator,
        device: &crate::Device,
        desc: TextureDescriber,
        mut previous_fame_end: NonSendMut<PreviousFrameEnd>,
    ) -> Self {
        let mut uploads = RecordingCommandBuffer::new(
            command_buffer_allocator.0.clone(),
            device.upload_queue().queue_family_index(),
            CommandBufferLevel::Primary,
            CommandBufferBeginInfo {
                usage: CommandBufferUsage::OneTimeSubmit,
                ..Default::default()
            },
        )
        .unwrap();

        let upload_buffer = Buffer::from_iter(
            memory_allocator.0.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            desc.bytes.iter().cloned(),
        )
        .unwrap();

        let image = Image::new(
            memory_allocator.0.clone(),
            ImageCreateInfo {
                image_type: desc.image_type,
                format: desc.format,
                extent: desc.extent,
                array_layers: desc.array_layers,
                usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
                ..Default::default()
            },
            AllocationCreateInfo::default(),
        )
        .unwrap();

        uploads
            .copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
                upload_buffer,
                image.clone(),
            ))
            .unwrap();

        previous_fame_end.submit_buffer(uploads, device.upload_queue().clone());

        TextureView(ImageView::new_default(image).unwrap())
    }
}
