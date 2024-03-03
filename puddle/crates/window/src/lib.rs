#![allow(unused, dead_code)]
pub mod event_list;
mod event_runner;
use legion::Resources;
pub use winit;
mod input;
pub use input::InputList;

use application::{Application, Plugin};
use std::{collections::HashMap, sync::Arc};

use winit::{event_loop::EventLoop, window::Window};

pub struct WindowPlugin;

pub struct PuddleWindow {
    pub window: Arc<Window>,
}

impl Plugin for WindowPlugin {
    fn build(&mut self, app: &mut Application) {
        let event_loop = EventLoop::new().unwrap();
        let window = Window::new(&event_loop).unwrap();

        let puddle_window = PuddleWindow {
            window: Arc::new(window),
        };

        app.resources.insert(puddle_window);
        app.resources.insert(event_loop);


        use events::EventHandler;
        app.resources.insert( EventHandler::<event_list::Resize>::new() );
        app.resources.insert( EventHandler::<event_list::DeviceEvent>::new() );

        app.resources.insert(InputList(HashMap::new(), Resources::default()));

        app.runner = Some(Box::new(event_runner::runner));
    }
}
