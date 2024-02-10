use application::Application;
use std::sync::{Arc, Mutex};
use winit::error::EventLoopError;
use winit::event_loop::{EventLoop, EventLoopBuilder, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};

pub struct WindowPlugin;
pub struct PuddleWindow(Arc<Window>);
pub struct PuddleEventLoop<T: 'static>(EventLoop<T>);

impl application::Plugin for WindowPlugin {
    fn build(&mut self, app: &mut Application) {
        let window: Window = WindowBuilder::new()
            .with_title("Puddle Application")
            .with_theme(Some(winit::window::Theme::Dark))
            .build(&app.event_loop)
            .expect("failed to create window");

        let arc_window = PuddleWindow(Arc::new(window));
        app.resources.insert(arc_window);
    }
}

impl PuddleWindow {
    pub fn set_tite(&self, title: &str) {
        self.0.set_title(title)
    }

    pub fn get(&self) -> &Arc<Window> {
        &self.0
    }

    pub fn inner_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.0.inner_size()
    }

    pub fn clone(&self) -> Self {
        PuddleWindow(self.0.clone())
    }

    pub fn get_cloned(&self) -> Arc<Window> {
        self.0.clone()
    }
}
