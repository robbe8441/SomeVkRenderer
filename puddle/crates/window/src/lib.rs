#![allow(unused, dead_code)]
mod event_runner;
mod send_events;

pub use winit;
pub use send_events::events;

use application::{Application, Plugin};
use legion::Resources;
use std::sync::Arc;
use winit::{event_loop::EventLoop, window::Window};

pub struct WindowPlugin;

pub struct PuddleWindow {
    pub window: Arc<Window>,
}

impl Plugin for WindowPlugin {
    fn build(&mut self, app: &mut Application) {
        let event_loop = EventLoop::new().unwrap();

        let window = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();

        let puddle_window = PuddleWindow {
            window: Arc::new(window),
        };

        app.resources.insert(puddle_window);
        app.resources.insert(event_loop);

        app.runner = Some(Box::new(event_runner::runner));
    }
}
