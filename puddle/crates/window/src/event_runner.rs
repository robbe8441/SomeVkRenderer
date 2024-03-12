use std::{collections::HashMap, time::Instant};

use application::Application;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

use crate::PuddleWindow;

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
        .run(move |mut event, target| {
            crate::send_events::handle_events(&event, app);

            match event {
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

                Event::AboutToWait => window.request_redraw(),
                _ => {}
            }
        })
        .unwrap();
}
