#![allow(unused, dead_code)]
mod event_runner;
use legion::Resources;
pub use winit;

use application::{Application, Plugin};
use std::{collections::HashMap, sync::Arc};

use winit::{event_loop::EventLoop, window::Window};

pub struct WindowPlugin;

pub struct PuddleWindow {
    pub window: Arc<Window>,
}

pub struct WindowEventHandler {
    pub handler : events::EventHandler<winit::event::Event<()>>
}

impl Plugin for WindowPlugin {
    fn build(&mut self, app: &mut Application) {
        let event_loop = EventLoop::new().unwrap();
        let window = Window::new(&event_loop).unwrap();

        let puddle_window = PuddleWindow {
            window: Arc::new(window),
        };

        let event_handler = WindowEventHandler {
            handler : events::EventHandler::new(),
        };

        app.resources.insert(puddle_window);
        app.resources.insert(event_loop);
        app.resources.insert(event_handler);

        use events::EventHandler;
        app.runner = Some(Box::new(event_runner::runner));
    }
}
