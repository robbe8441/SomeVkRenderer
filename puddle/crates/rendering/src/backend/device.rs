use std::sync::Arc;

use bevy_ecs::system::{Commands, NonSendMut, Res, Resource};
use vulkano::device::{
    physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Queue,
    QueueCreateInfo, QueueFlags,
};
use vulkano::sync::GpuFuture;

use crate::PreviousFrameEnd;

use super::instance::RenderInstance;
use super::surface::RenderSurface;

#[allow(unused)]
#[derive(Resource)]
pub struct RenderDevice {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
}

pub fn create_device(
    instance: Res<RenderInstance>,
    surface: Res<RenderSurface>,
    mut previous_frame_end: NonSendMut<PreviousFrameEnd>,
    mut commands: Commands,
) {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };
    let (physical_device, queue_family_index) = instance
        .0
        .enumerate_physical_devices()
        .unwrap()
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.intersects(QueueFlags::GRAPHICS)
                        && p.surface_support(i as u32, &surface.0).unwrap_or(false)
                })
                .map(|i| (p, i as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .unwrap();

    println!(
        "Using device: {} (type: {:?})",
        physical_device.properties().device_name,
        physical_device.properties().device_type,
    );

    let (device, queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: device_extensions,
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .unwrap();

    let render_device = RenderDevice {
        device,
        queue: queues.into_iter().last().unwrap(),
    };

    previous_frame_end.0 = Some(vulkano::sync::now(render_device.device.clone()).boxed());

    commands.insert_resource(render_device)
}
