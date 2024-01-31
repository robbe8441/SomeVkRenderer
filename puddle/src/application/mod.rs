use std::{time::Instant, collections::HashMap};

pub use super::*;
pub mod window_handler;



pub struct Application {
    pub on_startup : core::events::EventDispatcher<()>,

    pub window : window_handler::ApplicationWindow,
    event_loop : winit::event_loop::EventLoop<()>,
    time : Instant,

    storrage : HashMap<String, f64>
}

impl Application {

    pub fn new() -> Self {
        logger::init();

        let (window, event_loop) = window_handler::ApplicationWindow::new();

        Self { 
            on_startup : core::events::EventDispatcher::new(),
            time : Instant::now(),
            window,
            event_loop,
            storrage : HashMap::default(),
        }
    }

    pub fn set_variable(&mut self, name : String, value : f64) {
        self.storrage.insert(name, value);
    }
    
    pub fn get_variable(&mut self, name : String) -> Option<&f64> {
        self.storrage.get(&name)
    }

    pub fn run(mut self) {
        self.on_startup.fire(&());

        self.event_loop.run(move | event, target | {
            self.window.handle_event(&event, &target, self.time);

        }).unwrap();
    }
}
