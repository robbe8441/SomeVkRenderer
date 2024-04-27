pub mod events;

use application::Application;
use winit::event::WindowEvent;
use winit::{
    event::{DeviceEvent, Event, RawKeyEvent},
    keyboard::KeyCode,
};

pub fn handle_events(event: &Event<()>, app: &mut Application) {
    match event {
        Event::WindowEvent { window_id:_, event } => match event {
            _=> {}
        }

        _ => {}
    }
}
