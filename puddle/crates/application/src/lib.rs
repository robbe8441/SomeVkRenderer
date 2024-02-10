mod plugin;
pub use legion::systems::CommandBuffer;
pub use plugin::Plugin;

use legion::systems::ParallelRunnable;
use legion::*;
use logger::info;
use std::sync::Arc;
pub use tokio;
use tokio::time::Instant;
use winit::event::{Event, WindowEvent};

pub enum ScheduleRunMode {
    Update,
    Startup,
}

pub struct PuddleSchedule {
    pub on_update: systems::Builder,
    pub on_startup: systems::Builder,
}

pub struct Application {
    pub world: World,
    pub resources: Resources,
    pub event_loop: winit::event_loop::EventLoop<()>,
    pub on_event: events::EventDispatcher<winit::event::Event<()>>,

    pub schedule: PuddleSchedule,
    plugins: Option<plugin::PluginHandler>,
}
pub struct DeltaTime(pub f64);

impl Application {
    pub fn new() -> Self {
        logger::init();
        let world = World::default();
        let mut resources = Resources::default();

        let schedule = PuddleSchedule {
            on_startup: Schedule::builder(),
            on_update: Schedule::builder(),
        };

        let tokio_runtime = tokio::runtime::Runtime::new().expect("failed to create runtime");
        resources.insert(Arc::new(tokio_runtime));

        let plugins = plugin::PluginHandler::new();
        let event_loop = winit::event_loop::EventLoop::new().unwrap();

        Self {
            world,
            schedule,
            resources,
            on_event: events::EventDispatcher::new(),
            plugins: Some(plugins),
            event_loop,
        }
    }

    pub fn add_system<T: ParallelRunnable + 'static>(
        &mut self,
        mode: ScheduleRunMode,
        system: T,
    ) -> &mut Self {
        match mode {
            ScheduleRunMode::Startup => self.schedule.on_startup.add_system(system),
            ScheduleRunMode::Update => self.schedule.on_update.add_system(system),
        };
        self
    }

    pub fn add_plugin(&mut self, plugin: impl Plugin + 'static) {
        if let Some(mut plugins) = self.plugins.take() {
            plugins.add_plugin(plugin);
            self.plugins = Some(plugins);
        }
    }

    pub fn run(mut self) {
        self.resources.insert(Instant::now());
        self.resources.insert(DeltaTime(0.0));

        let command_buffer = legion::systems::CommandBuffer::new(&self.world);
        self.resources.insert(command_buffer);

        if let Some(mut plugins) = self.plugins.take() {
            plugins.build(&mut self);
            self.plugins = Some(plugins);
        }

        let mut startup = self.schedule.on_startup.build();
        let mut update_schedule = self.schedule.on_update.build();

        startup.execute(&mut self.world, &mut self.resources);

        use winit::event::Event;
        use winit::event::WindowEvent;
        self.event_loop
            .run(move |mut event, target| {

                self.on_event.fire(&event);

                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        window_id: _,
                    } => target.exit(),
                    Event::AboutToWait => {
                        let start = Instant::now();
                        info!("about to wait");
                        update_schedule.execute(&mut self.world, &mut self.resources);

                        let buffer = self.resources.remove::<legion::systems::CommandBuffer>();

                        if let Some(mut command_buffer) = buffer {
                            command_buffer.flush(&mut self.world, &mut self.resources);
                            self.resources.insert(command_buffer);
                        }

                        self.resources
                            .insert(DeltaTime(start.elapsed().as_secs_f64()));
                    }
                    _ => {}
                }
            })
            .unwrap();
    }
}
