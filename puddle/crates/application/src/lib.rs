#![allow(unused, dead_code)]
use std::ops::Deref;
use std::sync::{Arc, Mutex};

pub use log;
pub use geese;
pub use legion;
pub use async_std;
use geese::EventQueue;
pub use scheddules::Scheddules;
pub use plugins::{Plugin, PluginHandler};

mod logger;
mod plugins;
mod scheddules;

use geese::GeeseContext;

pub struct Application {
    pub world: legion::World,
    pub resources: legion::Resources,
    pub plugins: Option<plugins::PluginHandler>,
    pub scheddules : scheddules::SchedduleHandler,
    pub runner: Option<Box<dyn FnOnce(&mut Application)>>,
    pub geese_context : GeeseContext,
}

impl Application {
    pub fn new() -> Self {
        logger::init();

        let mut app = Self {
            runner: None,
            plugins: None,
            world: legion::World::default(),
            resources: legion::Resources::default(),
            scheddules : scheddules::SchedduleHandler::new(),
            geese_context : GeeseContext::default(),
        };

        app.scheddules.get_or_add(Scheddules::Update);
        app.scheddules.get_or_add(Scheddules::Startup);

        app
    }

    pub fn add_event_listener<T: geese::GeeseSystem>(&mut self) -> &mut Self {
        self.geese_context.flush().with(geese::notify::add_system::<T>());
        self
    }

    pub fn add_plugin(&mut self, plugin: impl Plugin + 'static) -> &mut Self {
        self.plugins
            .get_or_insert_with(|| PluginHandler::new())
            .plugins
            .push(Box::new(plugin));
        self
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
