mod event_runner;

use bevy_ecs::system::Resource;
use winit;

use application::{Application, Plugin};
use std::sync::Arc;
use winit::event_loop;

pub struct WindowPlugin;

#[derive(Resource, Clone)]
pub struct Window(pub Arc<winit::window::Window>);

pub struct EventLoop(pub event_loop::EventLoop<()>);

impl Plugin for WindowPlugin {
    fn build(&mut self, app: &mut Application) {

        let event_loop = event_loop::EventLoop::new().unwrap();

        let window = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();

        let puddle_window = Window(Arc::new(window));

        app.world.insert_resource(puddle_window);
        app.world.insert_non_send_resource(EventLoop(event_loop));

        app.runner = Some(Box::new(event_runner::runner));
    }
}

impl Window {
    pub fn visible(&self) -> bool {
        self.0.is_visible().unwrap_or(false)
    }
}
