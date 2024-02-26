#[derive(Clone)]
pub struct Resize(pub winit::dpi::PhysicalSize<u32>);
pub struct DeviceEvent(pub winit::event::DeviceEvent);
