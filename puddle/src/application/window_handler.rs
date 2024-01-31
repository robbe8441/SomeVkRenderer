#![allow(unused)]

use std::time::Instant;

use log::info;
use super::core::events::EventDispatcher;
use winit::{
    event::{Event, WindowEvent, self},
    event_loop::EventLoop,
    window::{self, Window}, dpi::Size,
};

pub struct ApplicationWindow {
    pub size: (u32, u32),
    pub window: Window,
    pub on_resize_event : EventDispatcher<winit::dpi::PhysicalSize<u32>>,
    pub pre_render_event : EventDispatcher<f64>,
    pub on_close_requested : EventDispatcher,

    window_id : winit::window::WindowId,
    last_render : f64,
}

impl ApplicationWindow {
    pub fn new() -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new().unwrap();

        let builder = window::WindowBuilder::new()
            .with_title("Application")
            .with_theme(Some(window::Theme::Dark));

        let window = builder.build(&event_loop).unwrap();
        window.id();
        let size = window.inner_size();

        let app_window = Self {
            on_resize_event : EventDispatcher::new(),
            pre_render_event : EventDispatcher::new(),
            on_close_requested : EventDispatcher::new(),
            size: (size.width, size.height),

            window_id : window.id(),
            window,
            last_render : 0.0,
        };

        (app_window, event_loop)
    }

    pub fn handle_event(
        &mut self,
        event: &Event<()>,
        target: &winit::event_loop::EventLoopWindowTarget<()>,
        time : Instant
    ) {

        match event {

            Event::WindowEvent { event : WindowEvent::CloseRequested, .. } => {
                target.exit();
                self.on_close_requested.fire(&());
            }

            Event::WindowEvent { event : WindowEvent::RedrawRequested, .. } => {
                let time = time.elapsed().as_secs_f64();
                let delta_time = time - self.last_render;
                self.last_render = time;
                self.pre_render_event.fire(&delta_time);
            }

            Event::WindowEvent { event : WindowEvent::Resized(size), .. } => {
                self.size = (size.height, size.width);
                self.window.request_redraw();
                self.on_resize_event.fire(&size);
            }

            Event::AboutToWait => {
                self.window.request_redraw();
            }

            Event::LoopExiting => info!("window closed"),

            _ => {}
        }
    }

}

impl ApplicationWindow {

    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

}
