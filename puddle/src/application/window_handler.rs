#![allow(unused)]

use std::{sync::Arc, time::Instant};

use super::core::events::EventDispatcher;
use log::info;
use winit::{
    dpi::Size,
    event::{self, Event, WindowEvent},
    event_loop::EventLoop,
    window::{self, Window},
};

pub struct ApplicationWindow {
    pub window: Arc<Window>,

    pub on_resize_event: EventDispatcher<winit::dpi::PhysicalSize<u32>>,
    pub pre_render_event: EventDispatcher<f64>,
    pub on_close_requested: EventDispatcher,

    window_id: winit::window::WindowId,
    last_render: f64,
}

impl ApplicationWindow {
    pub fn new(tokio_runtime: &tokio::runtime::Runtime) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new().unwrap();

        let builder = window::WindowBuilder::new()
            .with_title("Application")
            .with_theme(Some(window::Theme::Dark));

        let window = Arc::new(builder.build(&event_loop).unwrap());
        window.id();
        let size = window.inner_size();

        let app_window = Self {
            on_resize_event: EventDispatcher::new(),
            pre_render_event: EventDispatcher::new(),
            on_close_requested: EventDispatcher::new(),

            window_id: window.id(),
            window: window.into(),
            last_render: 0.0,
        };

        (app_window, event_loop)
    }

    pub fn handle_event(
        &mut self,
        event: &Event<()>,
        target: &winit::event_loop::EventLoopWindowTarget<()>,
        time: Instant,
    ) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                self.on_close_requested.fire(&());
                target.exit();
            }

            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let time = time.elapsed().as_secs_f64();
                let delta_time = time - self.last_render;
                self.last_render = time;
                self.pre_render_event.fire(&delta_time);
            }

            Event::AboutToWait => {
                self.window.request_redraw();
            }

            Event::LoopExiting => info!("window closed"),

            _ => {}
        }
    }

    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }
}
