#![allow(unused, dead_code)]

use application::{Application, Plugin};
use events::EventHandler;
use legion::system;
use std::collections::HashMap;
use window::winit::{
    event::{DeviceEvent, KeyEvent, RawKeyEvent, WindowEvent},
    keyboard::PhysicalKey,
};

use std::sync::{Arc, Mutex};

pub enum InputEvents {
    MouseMoved(window::winit::dpi::PhysicalPosition<f64>),
    MouseDelta(f64, f64),
    KeyPressed(window::winit::keyboard::KeyCode, bool),
}

pub struct InputPlugin;

pub type ArcMut<T> = Arc<Mutex<T>>;

pub struct Input {
    keys_pressed: HashMap<window::winit::keyboard::KeyCode, bool>,
    pub event_handler: EventHandler<InputEvents>,
}
unsafe impl Send for Input {}

impl Plugin for InputPlugin {
    fn finish(&mut self, app: &mut Application) {
        let input = Input {
            keys_pressed: HashMap::new(),
            event_handler: EventHandler::new(),
        };

        app.scheddules
            .add_non_parralel(application::Scheddules::Startup, event_listen);
        app.resources.insert(Arc::new(Mutex::new(input)));
    }
}

impl Input {
    pub fn key_pressed(&self, key: window::winit::keyboard::KeyCode) -> bool {
        if let Some(v) = self.keys_pressed.get(&key) {
            return *v;
        }
        false
    }
}

fn event_listen(_world: &mut legion::World, resources: &mut legion::Resources) {
    let input = resources.get::<ArcMut<Input>>().unwrap().clone();
    let mut window_events = resources.get_mut::<window::WindowEventHandler>().unwrap();

    use window::winit::event::{DeviceEvent, Event, WindowEvent};

    window_events.handler.connect(move |event| match event {
        Event::DeviceEvent {
            device_id: _,
            event,
        } => match event {
            DeviceEvent::MouseMotion { delta } => {
                input
                    .lock()
                    .unwrap()
                    .event_handler
                    .fire(&mut InputEvents::MouseDelta(delta.0, delta.1));
            }
            DeviceEvent::Key(RawKeyEvent {
                physical_key,
                state,
            }) => {
                if let PhysicalKey::Code(code) = physical_key {
                    let mut input = input.lock().unwrap();
                    input
                        .event_handler
                        .fire(&mut InputEvents::KeyPressed(*code, state.is_pressed()));
                    input.keys_pressed.insert(*code, state.is_pressed());
                }
            }

            _ => {}
        },

        Event::WindowEvent {
            window_id: _,
            event,
        } => match event {
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                input
                    .lock()
                    .unwrap()
                    .event_handler
                    .fire(&mut InputEvents::MouseMoved(*position));
            }

            WindowEvent::KeyboardInput {
                device_id: _,
                event:
                    KeyEvent {
                        physical_key,
                        state,
                        ..
                    },
                ..
            } => {
                if let PhysicalKey::Code(code) = physical_key {
                    let mut input = input.lock().unwrap();
                    input
                        .event_handler
                        .fire(&mut InputEvents::KeyPressed(*code, state.is_pressed()));
                    input.keys_pressed.insert(*code, state.is_pressed());
                }
            }
            _ => {}
        },

        _ => {}
    });
}
