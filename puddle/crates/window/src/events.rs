use bevy_ecs::event::Event;
use winit::dpi::PhysicalSize;


#[derive(Event)]
pub struct Resized(pub PhysicalSize<u32>);


