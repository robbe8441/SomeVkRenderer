use application::Application;
use winit::event::{Event, WindowEvent};
use crate::events;

use crate::Window;

pub(crate) fn runner(mut app: Application) {
    setup(&mut app);

    let event_loop = app
        .world
        .remove_non_send_resource::<crate::EventLoop>()
        .expect("failed to get event_loop")
        .0;

    event_loop
        .run(move |event, target| {
            // crate::send_events::handle_events(&event, app);

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => target.exit(),

                Event::WindowEvent {
                    window_id: _,
                    event: WindowEvent::RedrawRequested,
                } => {
                    // update the game loop
                    app.schedules.get_mut(application::PreUpdate).map(|r| {
                        r.run(&mut app.world);
                    });

                    app.schedules.get_mut(application::Update).map(|r| {
                        r.run(&mut app.world);
                    });

                    app.schedules.get_mut(application::PostUpdate).map(|r| {
                        r.run(&mut app.world);
                    });
                }

                Event::WindowEvent {
                    window_id: _,
                    event: WindowEvent::Resized(size),
                } => {
                    
                    app.world.send_event(events::Resized(size));

                }

                Event::AboutToWait => {
                    app.world.get_resource::<Window>().map(|window| {
                        window.0.request_redraw();
                    });
                }
                _ => {}
            }
        })
        .unwrap();
}

#[inline]
fn setup(app: &mut Application) {
    app.schedules
        .get_mut(application::PreStartup)
        .map(|r| r.run(&mut app.world));

    app.schedules
        .get_mut(application::Startup)
        .map(|r| r.run(&mut app.world));

    app.schedules
        .get_mut(application::PostStartup)
        .map(|r| r.run(&mut app.world));
}
