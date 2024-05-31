use std::sync::Arc;

use super::instance::Instance;
use application::log::info;
use bevy_ecs::system::Resource;
use vulkano::device::{
    physical::PhysicalDeviceType, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo, QueueFlags,
};

#[derive(Resource)]
pub struct Device {
    pub device: Arc<vulkano::device::Device>,
    pub queues: Arc<[Arc<vulkano::device::Queue>]>,
}

#[rustfmt::skip]
const QUEUES: [(f32, QueueFlags); 3] = [
    (0.7, QueueFlags::GRAPHICS), 
    (0.5, QueueFlags::COMPUTE),
    (0.3, QueueFlags::TRANSFER),
];

#[allow(unused, dead_code)]
impl Device {
    pub fn new(instance: &Instance, surface: &super::Surface) -> Self {
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };
        let physical_device = instance
            .0
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .min_by_key(|p| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            })
            .unwrap();

        let mut used_queue_families = vec![];

        let queue_create_infos = QUEUES
            .iter()
            .map(|(priority, flags)| {
                let index = physical_device
                    .queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.intersects(*flags)
                            && physical_device
                                .surface_support(i as u32, &surface.0)
                                .unwrap_or(false)
                            && !used_queue_families.contains(&i)
                    })
                    .unwrap();
                used_queue_families.push(index);
                (index as u32, *priority)
            })
            .map(|(queue_family_index, priority)| QueueCreateInfo {
                queue_family_index,
                queues: vec![priority],
                ..Default::default()
            })
            .collect();

        info!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );

        let (device, queues) = vulkano::device::Device::new(
            physical_device,
            DeviceCreateInfo {
                enabled_extensions: device_extensions,
                queue_create_infos,
                ..Default::default()
            },
        )
        .unwrap();

        Self {
            device,
            queues: queues.collect::<Vec<Arc<vulkano::device::Queue>>>().into(),
        }
    }

    #[inline(always)]
    pub fn render_queue(&self) -> &Arc<vulkano::device::Queue> {
        self.queues.first().unwrap()
    }

    #[inline(always)]
    pub fn upload_queue(&self) -> &Arc<vulkano::device::Queue> {
        self.queues.get(1).unwrap_or(self.render_queue())
    }

    #[inline(always)]
    pub fn compute_queue(&self) -> &Arc<vulkano::device::Queue> {
        self.queues.get(2).unwrap_or(self.upload_queue())
    }
}
