use std::{collections::HashMap, time::Instant};
use crate::input::InputList;

use application::Application;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

use crate::event_list;
use crate::PuddleWindow;
use events::EventHandler;


pub(crate) fn runner(app: &mut Application) {
    let event_loop = app
        .resources
        .remove::<EventLoop<()>>()
        .expect("failed to get event_loop");
    let mut update_schedule = app
        .scheddules
        .remove(application::Scheddules::Update)
        .unwrap()
        .build();
    let mut startup_schedule = app
        .scheddules
        .remove(application::Scheddules::Startup)
        .unwrap()
        .build();

    let window = app.resources.get::<PuddleWindow>().unwrap().window.clone();

    startup_schedule.execute(&mut app.world, &mut app.resources);

    event_loop
        .run(move |event, target| {
            match event {
                Event::DeviceEvent {
                    device_id: _,
                    event,
                } => {
                    use winit::event::{DeviceEvent, RawKeyEvent};
                    use winit::keyboard::PhysicalKey;

                    if let DeviceEvent::Key(RawKeyEvent {
                        physical_key,
                        state,
                    }) = event
                    {
                        if let Some(mut list) = app.resources.get_mut::<InputList>() {
                            list.0.insert(physical_key, state.is_pressed());
                        }
                    }
                }

                Event::WindowEvent {
                    window_id: _,
                    event: WindowEvent::CloseRequested,
                } => target.exit(),

                Event::WindowEvent {
                    window_id: _,
                    event: WindowEvent::RedrawRequested,
                } => {
                    // update the game loop
                    update_schedule.execute(&mut app.world, &mut app.resources);
                }

                Event::WindowEvent {
                    window_id: _,
                    event: WindowEvent::Resized(size),
                } => {
                    if let Some(event) = app.resources.get::<EventHandler<event_list::Resize>>() {
                        event.fire(&mut event_list::Resize(size));
                    }
                }

                Event::AboutToWait => window.request_redraw(),

                _ => {}
            }
        })
        .unwrap();
}
