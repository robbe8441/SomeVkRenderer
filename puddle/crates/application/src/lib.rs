use std::sync::Arc;
use tokio::time::Instant;
use legion::*;
use legion::systems::ParallelRunnable;
use window::winit::keyboard::KeyCode::Insert;

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
    pub tokio_runtime: tokio::runtime::Runtime,

    pub renderer : rendering::Renderer,
}

impl Application {
    pub fn new() -> Self {
        logger::init();
        let world = World::default();
        let resources = Resources::default();

        let schedule = PuddleSceddule {
            on_startup : Schedule::builder(),
            on_update : Schedule::builder(),
        };

        let (window, event_loop) = window::new();
        let tokio_runtime = tokio::runtime::Runtime::new().expect("failed to create runtime");
        let renderer = rendering::Renderer::new(window.clone(), &tokio_runtime);

        Self { world, schedule, resources, window, event_loop, tokio_runtime, renderer }
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
        startup.execute(&mut self.world, &mut self.resources );
        let mut update_schedule = self.schedule.on_update.build();

        use window::Event;
        self.event_loop.run(move |event, target| {
            match event {
                Event::AboutToWait => self.window.request_redraw(),

                Event::WindowEvent {
                    window_id : _,
                    event : window::WindowEvent::RedrawRequested,
                } => {
                    update_schedule.execute(&mut self.world, &mut self.resources);
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
