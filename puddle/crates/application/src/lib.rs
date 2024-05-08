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
    pub runner: Option<Box<dyn FnOnce(Application)>>,
    pub schedules: Schedules,
}

impl Application {
    pub fn empty() -> Self {
        logger::init();

        let mut app = Self {
            runner: None,
            plugins: Some(PluginHandler::new()),
            world: World::new(),
            schedules: Schedules::new(),
        };

        app
    }

    pub fn add_plugin(&mut self, plugin: impl Plugin + 'static) -> &mut Self {
        let mut handler = self.plugins.take().unwrap_or(PluginHandler::new());
        handler.plugins.push(Box::new(plugin));
        self.plugins = Some(handler);

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
            .get_mut(label.clone())
            .expect("cant find schedule");

        schedule.add_systems(systems);
    }

    pub fn run_systems(&mut self, label: impl ScheduleLabel + Clone) {
        let mut schedule = self.schedules.get_mut(label).unwrap();
        schedule.run(&mut self.world);
    }

    pub fn run(mut self) {

        if let Some(mut plugins) = self.plugins.take() {
            plugins.build(&mut self);
            self.plugins = Some(plugins);
        }

        if let Some(runner) = self.runner.take() {
            (runner)(self)
        }
    }
}


impl Default for Application {

    fn default() -> Self {
        let mut app = Application::empty();

        SchedulePlugin::build(&mut SchedulePlugin, &mut app);

        app
    }

}
