use crate::Application;
use std::any::Any;
use std::collections::HashMap;

pub trait Plugin {
    fn build(&mut self, app: &mut Application) {}
    fn second_build_stage(&mut self, app: &mut Application) {}
    fn finish(&mut self, app: &mut Application) {}
    fn clean(self)
    where
        Self: Sized,
    {
    }
}

pub(crate) struct PluginHandler {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginHandler {
    pub fn new() -> Self {
        Self { plugins: vec![] }
    }

    pub fn add_plugin(&mut self, plugin: impl Plugin + 'static) {
        self.plugins.push(Box::new(plugin));
    }

    pub fn build(&mut self, app: &mut Application) {
        for plugin in self.plugins.iter_mut() {
            plugin.build(app);
        }
        for plugin in self.plugins.iter_mut() {
            plugin.second_build_stage(app);
        }
        for plugin in self.plugins.iter_mut() {
            plugin.finish(app);
        }
    }
}
