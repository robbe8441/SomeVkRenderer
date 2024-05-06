use std::time::Instant;

use application::Plugin;
use bevy_ecs::system::{ResMut, Resource};


pub struct TimePlugin;

#[allow(unused)]
#[derive(Resource)]
pub struct Time {
    pub delta: f32,
    pub delta64: f64,
    pub startup: Instant,
    pub last_frame: Instant,
}


impl Plugin for TimePlugin {
    fn build(&mut self, app: &mut application::Application) {

        app.world.insert_resource(Time {
            delta: 0.0,
            delta64: 0.0,
            startup: Instant::now(),
            last_frame: Instant::now(),
        });

        app.add_systems(application::Update, record_delta_time)
    }
}

fn record_delta_time(
    mut time: ResMut<Time>,
) {
    let dt = time.last_frame.elapsed().as_secs_f64();
    time.last_frame = Instant::now();
    time.delta64 = dt;
    time.delta = dt as f32;
}
