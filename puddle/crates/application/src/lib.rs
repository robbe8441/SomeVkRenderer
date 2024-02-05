
use std::sync::Arc;
use tokio::time::Instant;
use legion::*;
use legion::systems::ParallelRunnable;
use logger::*;

pub mod rendering;
pub mod window;
pub use winit;
pub use wgpu;

pub enum ScheduleRunMode {
    Update,
    Startup
}

struct PuddleSceddule {
    pub on_update : legion::systems::Builder,
    pub on_startup : legion::systems::Builder,
}

pub struct Application {
    pub world : World,
    schedule : PuddleSceddule,
    pub  resources : Resources,

    event_loop : window::EventLoop<()>,
    pub window : Arc<window::window::Window>,

    renderer : rendering::Renderer,
    imgui : puddle_imgui::PuddleImGuiRenderer,
}

impl Application {
    pub fn new() -> Self {
        logger::init();
        let world = World::default();
        let mut resources = Resources::default();

        let schedule = PuddleSceddule {
            on_startup : Schedule::builder(),
            on_update : Schedule::builder(),
        };

        let (window, event_loop) = window::new();
        let tokio_runtime = tokio::runtime::Runtime::new().expect("failed to create runtime");
        let renderer = rendering::Renderer::new(window.clone(), &tokio_runtime);

        let imgui = puddle_imgui::PuddleImGuiRenderer::new(window.clone(), &renderer.device, &renderer.queue, &renderer.surface_desc);

        resources.insert(tokio_runtime);

        Self { world, schedule, resources, window, event_loop, renderer, imgui }
    }

    pub fn add_system<T : ParallelRunnable + 'static>(&mut self, mode : ScheduleRunMode, system : T) -> &mut Self {
        match mode {
            ScheduleRunMode::Startup => self.schedule.on_startup.add_system(system),
            ScheduleRunMode::Update => self.schedule.on_update.add_system(system)
        };
        self
    }

    pub fn run(mut self) {
       self.resources.insert(Instant::now());

        let mut startup = self.schedule.on_startup.build();
        let mut update_schedule = self.schedule.on_update.build();

        startup.execute(&mut self.world, &mut self.resources );

        use window::Event;
        self.event_loop.run(move |event, target| {

            self.imgui.platform.handle_event( &mut self.imgui.imgui.io_mut(), self.window.clone(), &event);

            match event {
                Event::AboutToWait => self.window.request_redraw(),

                    Event::WindowEvent {
                        window_id : _,
                        event : window::WindowEvent::Resized(size)
                    } => {
                        self.renderer.surface_desc.width = size.width;
                        self.renderer.surface_desc.height = size.height;
                        self.renderer.surface.configure( &self.renderer.device, &self.renderer.surface_desc);
                    }

                Event::WindowEvent {
                    window_id : _,
                    event : window::WindowEvent::RedrawRequested,
                } => {
                    update_schedule.execute(&mut self.world, &mut self.resources);

                    let mut render_pass = rendering::RenderContext::new(&self.renderer.device, &self.renderer.queue, &self.renderer.surface);
                    if let Some(mut render_pass) = render_pass {
                        self.imgui.draw_imgui(self.window.clone(), &mut render_pass.command_encoder, &mut render_pass.view, &self.renderer.device, &self.renderer.queue);

                        render_pass.draw();
                    }
                }

                Event::WindowEvent {
                    window_id : _,
                    event : window::WindowEvent::CloseRequested
                } => target.exit(),

                _ => {}
            }
        }).unwrap();
    }
}
