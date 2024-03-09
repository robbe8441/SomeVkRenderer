use events::EventHandler;
use std::sync::{Arc, Mutex};

pub fn init(events : &mut window::WindowEventHandler, render_events : Arc<Mutex<crate::RenderEvents>>) {
    let poll = render_events.clone();

    events.handler.connect(move |event| {
        match event {
            window::winit::event::Event::WindowEvent {
                window_id:_,
                event: window::winit::event::WindowEvent::Resized(size) } => {
                    poll.lock().unwrap().resized = Some(size.clone());
            }
            _ => {}
        }
    });
}
