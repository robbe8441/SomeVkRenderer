use std::time::Instant;

pub use super::*;
pub mod window_handler;

pub struct Application {
    pub window: window_handler::ApplicationWindow,
    pub tokio_runtime: tokio::runtime::Runtime,
    pub on_startup: core::events::EventDispatcher<()>,

    event_loop: winit::event_loop::EventLoop<()>,
    time: Instant,
    pub renderer: render::Renderer,
}

impl Application {
    pub fn new() -> Self {
        let app = Application::create_raw();

        return app;
    }

    fn create_raw() -> Self {
        let tokio_runtime = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");

        let (window, event_loop) = window_handler::ApplicationWindow::new(&tokio_runtime);

        let app_renderer = render::Renderer::new(&tokio_runtime, window.window.clone());

        Self {
            tokio_runtime,
            on_startup: core::events::EventDispatcher::new(),
            time: Instant::now(),
            event_loop,
            window,
            renderer: app_renderer,
        }
    }

    pub fn run(mut self) {
        self.on_startup.fire(&());

        self.event_loop
            .run(move |event, target| {
                self.window.handle_event(&event, &target, self.time);

                if self.renderer.imgui_render.is_some() {
                    let renderer =  self.renderer.imgui_render.as_mut().unwrap();
                    renderer.imgui.io_mut().update_delta_time(self.time.elapsed());
                    renderer.platform.handle_event(renderer.imgui.io_mut(), &self.window.window, &event);
                }

                match event {
                    winit::event::Event::WindowEvent {
                        event: winit::event::WindowEvent::Resized(size),
                        ..
                    } => {

                        self.renderer.surface_desc.width = size.width;
                        self.renderer.surface_desc.height = size.height;

                        self.renderer
                            .surface
                            .configure(&self.renderer.device, &self.renderer.surface_desc);
                    }

                    winit::event::Event::WindowEvent {
                        event: winit::event::WindowEvent::RedrawRequested,
                        ..
                    } => {
                        self.renderer.draw(self.window.window.clone());
                    }


                    

                    _ => {}
                }
            })
            .unwrap();
    }
}
