use std::sync::Arc;
pub use winit::{
    window::{self, Window},
    event::{self, Event, WindowEvent},
    event_loop::EventLoop,
};

pub use winit;

pub fn new() -> (Arc<Window>, EventLoop<()>) {
    let event_loop = EventLoop::new().unwrap();

    let builder = window::WindowBuilder::new()
        .with_title("Application")
        .with_theme(Some(window::Theme::Dark));

    let window = Arc::new(builder.build(&event_loop).unwrap());

    (window, event_loop)
}

