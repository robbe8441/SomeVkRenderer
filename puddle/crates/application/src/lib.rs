#![allow(unused, dead_code)]
use std::ops::Deref;

pub use legion;
pub use log;
pub use plugins::{Plugin, PluginHandler};
pub use async_std;
pub use scheddules::Scheddules;

mod logger;
mod plugins;
mod scheddules;

pub struct Application {
    pub world: legion::World,
    pub resources: legion::Resources,
    pub plugins: Option<plugins::PluginHandler>,
    pub runner: Option<Box<dyn FnOnce(&mut Application)>>,
    pub scheddules : scheddules::SchedduleHandler,
}

impl Application {
    pub fn new() -> Self {
        logger::init();

        let mut app = Self {
            world: legion::World::default(),
            resources: legion::Resources::default(),
            scheddules : scheddules::SchedduleHandler::new(),
            plugins: None,
            runner: None,
        };

        app.scheddules.get_or_add(Scheddules::Update);
        app.scheddules.get_or_add(Scheddules::Startup);

        app
    }

    pub fn add_plugin(&mut self, plugin: impl Plugin + 'static) {
        self.plugins
            .get_or_insert_with(|| PluginHandler::new())
            .plugins
            .push(Box::new(plugin));
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
