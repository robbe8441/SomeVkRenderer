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
        .schedules
        .remove(application::Schedules::Update)
        .unwrap()
        .build();
    let mut startup_schedule = app
        .schedules
        .remove(application::Schedules::Startup)
        .unwrap()
        .build();

    let window = app.resources.get::<PuddleWindow>().unwrap().window.clone();

    startup_schedule.execute(&mut app.world, &mut app.resources);

    let mut update_every_list : Vec<_> = app.schedules.list.iter_mut().filter_map(|(schedule,mut sys)| {
        match schedule {
            application::Schedules::UpdateEvery(time) => {
                Some((time.clone(), Instant::now(), sys.build()))
            }
            _=>{None}
        }
    }).collect();



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

                    // check for systems that need to update
                    for (time, last_update, system) in update_every_list.iter_mut() {
                        if last_update.elapsed().as_secs_f32() > time.as_secs_f32() {
                            *last_update = Instant::now();
                            system.execute(&mut app.world, &mut app.resources);
                        }
                    }


                }

                Event::AboutToWait => window.request_redraw(),
                _ => {}
            }
        })
        .unwrap();
}
