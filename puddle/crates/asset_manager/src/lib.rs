mod model;

use application::Update;

pub struct AssetManagerPlugin;

use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::Without,
    system::{Commands, NonSendMut, Query, Res},
};
pub use model::{ModelBundle, Vertices};
use rendering::{
    backend::{
        buffer::{CommandBufferAllocator, DescriptorSetAllocator, StandardMemoryAllocator},
        device::{PreviousFrameEnd, RenderDevice},
    },
    frontend::{
        types::{VoxelBuffer, VoxelDescriptorSet},
        PipelineSetup,
    },
    vulkano::{
        command_buffer::{
            CommandBufferBeginInfo, CommandBufferLevel, CommandBufferUsage, CopyBufferToImageInfo,
            RecordingCommandBuffer,
        },
        descriptor_set::{DescriptorSet, WriteDescriptorSet},
        image::{view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage},
        memory::allocator::AllocationCreateInfo,
        sync::GpuFuture,
    },
};

impl application::Plugin for AssetManagerPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        app.add_systems(Update, model::load_vertex_buffers);
        app.add_systems(Update, load_voxels);
    }
}

#[derive(Component)]
pub struct VoxelMesh {
    pub size: [u32; 3],
    pub data: Vec<u8>,
}

fn load_voxels(
    voxel_query: Query<(Entity, &VoxelMesh), Without<VoxelDescriptorSet>>,
    descriptor_set_allocator: Res<DescriptorSetAllocator>,
    memory_allocator: Res<StandardMemoryAllocator>,
    command_buffer_allocator: Res<CommandBufferAllocator>,
    pipeline: Res<PipelineSetup>,
    device: Res<RenderDevice>,
    mut previous_frame_end: NonSendMut<PreviousFrameEnd>,
    mut commadns: Commands,
) {
    if voxel_query.iter().count() == 0 {
        return;
    }

    println!("upload model");

    let mut uploads = RecordingCommandBuffer::new(
        command_buffer_allocator.0.clone(),
        device.queue.queue_family_index(),
        CommandBufferLevel::Primary,
        CommandBufferBeginInfo {
            usage: CommandBufferUsage::OneTimeSubmit,
            ..Default::default()
        },
    )
    .unwrap();

    for (entity, voxel_mesh) in voxel_query.iter() {
        let voxel_buffer =
            VoxelBuffer::new(&memory_allocator, voxel_mesh.data.clone(), voxel_mesh.size);

        use rendering::vulkano::pipeline::Pipeline;
        let layout = &pipeline.pipeline.layout().set_layouts()[1];

        let image = Image::new(
            memory_allocator.0.clone(),
            ImageCreateInfo {
                extent: voxel_mesh.size,
                image_type: ImageType::Dim3d,
                usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
                format: rendering::vulkano::format::Format::R8_UINT,
                ..Default::default()
            },
            AllocationCreateInfo {
                ..Default::default()
            },
        )
        .unwrap();

        uploads
            .copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
                voxel_buffer.buffer,
                image.clone(),
            ))
            .unwrap();

        let view = ImageView::new_default(image).unwrap();

        let set = DescriptorSet::new(
            descriptor_set_allocator.0.clone(),
            layout.clone(),
            [WriteDescriptorSet::image_view(0, view)],
            [],
        )
        .unwrap();

        commadns.entity(entity).insert(VoxelDescriptorSet(set));
    }

    previous_frame_end.0 = Some(
        uploads
            .end()
            .unwrap()
            .execute(device.queue.clone())
            .unwrap()
            .boxed(),
    );
}
