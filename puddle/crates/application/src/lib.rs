#![allow(unused, dead_code)]

pub use async_std;
pub use log;
pub use plugins::{Plugin, PluginHandler};
pub use schedule_handler::*;

use bevy_ecs::{prelude::*, schedule::ScheduleLabel};

mod logger;
mod plugins;
mod schedule_handler;

pub struct Application {
    pub world: World,
    pub plugins: Option<plugins::PluginHandler>,
    pub runner: Option<Box<dyn FnOnce(&mut Application)>>,
    pub schedules: Schedules,
}

impl Application {
    pub fn new() -> Self {
        logger::init();

        let mut app = Self {
            runner: None,
            plugins: None,
            world: World::new(),
            schedules: Schedules::new(),
        };

        app.runner = Some(Box::new(|_app| {}));

        app
    }

    pub fn add_plugin(&mut self, plugin: impl Plugin + 'static) -> &mut Self {
        self.plugins
            .get_or_insert_with(|| PluginHandler::new())
            .plugins
            .push(Box::new(plugin));
        self
    }

    pub fn get_schedule(&mut self, schedule: Schedule) {
        self.schedules.insert(schedule);
    }

    pub fn add_resource(&mut self, value: impl Resource) {
        self.world.insert_resource(value);
    }
    pub fn get_or_insert_resource<R: Resource>(&mut self, value: R) -> &R {
        self.world.insert_resource(value);
        self.world.get_resource::<R>().unwrap()
    }

    pub fn add_systems<M>(
        &mut self,
        label: impl ScheduleLabel + Clone,
        systems: impl IntoSystemConfigs<M>,
    ) {
        let mut schedule = self
            .schedules
            .remove(label.clone())
            .unwrap_or(Schedule::new(label));
        schedule.add_systems(systems);
        self.schedules.insert(schedule);
    }

    pub fn run_systems(&mut self, label: impl ScheduleLabel + Clone) {
        let mut schedule = self.schedules.get_mut(label).unwrap();
        schedule.run(&mut self.world);
    }

    pub fn run(mut self) {
        let mut plugins = self.plugins.take();

        if let Some(ref mut plugins) = plugins {
            log::info!("setting up plugins");
            plugins.build(&mut self);
        }

        if let Some(runner) = self.runner.take() {
            (runner)(&mut self)
        }

        if let Some(mut plugins) = plugins {
            log::info!("cleaning up");
            plugins.cleanup(&mut self);
        }
    }
}
