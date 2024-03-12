pub mod events;
use application::geese::*;

use application::Application;
use winit::event::WindowEvent;
use winit::{
    event::{DeviceEvent, Event, RawKeyEvent},
    keyboard::KeyCode,
};

pub fn handle_events(event: &Event<()>, app: &mut Application) {
    match event {
        Event::DeviceEvent {
            device_id: _,
            event,
        } => match event {
            DeviceEvent::Key(RawKeyEvent {
                physical_key,
                state,
            }) => {
                if let winit::keyboard::PhysicalKey::Code(key) = physical_key {
                    app.geese_context.flush().with(events::KeybordInput {
                        key : key.clone(),
                        state: state.is_pressed(),
                    });
                }
            }

            _ => {}
        }


        Event::WindowEvent { window_id:_, event } => match event {

            WindowEvent::Resized(size) => {
                app.geese_context.flush().with(events::ResizeWindow(size.clone()));
            }

            _=> {}
        }

        _ => {}
    }
}
