use std::time::Instant;

use application::Plugin;


pub struct TimePlugin;

#[allow(unused)]
pub struct Time {
    pub delta: f32,
    pub startup: Instant,
}


impl Plugin for TimePlugin {
    fn build(&mut self, app: &mut application::Application) {

        app.resources.insert(Time {
            delta: 0.0,
            startup: Instant::now(),
        });

        app.schedules.add(application::Schedules::Update, record_delta_time_system(Instant::now()));
    }
}

#[legion::system]
fn record_delta_time(
    #[state] last_update: &mut Instant,
    #[resource] time: &mut Time
) {
    let dt = last_update.elapsed().as_secs_f32();
    *last_update = Instant::now();

    time.delta = dt;
}
