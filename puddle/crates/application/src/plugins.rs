use super::Application;

pub trait Plugin {
    fn build(&mut self, app: &mut Application) {}
    fn finish(&mut self, app: &mut Application) {}
    fn cleanup(&mut self, app: &mut Application) {}
}

pub struct PluginHandler {
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl PluginHandler {
    pub(crate) fn new() -> Self {
        Self { plugins: vec![] }
    }

    /// setup the plugins
    pub(crate) fn build(&mut self, app: &mut Application) {

        for plugin in self.plugins.iter_mut() {
            plugin.build(app);
        }

        for plugin in self.plugins.iter_mut() {
            plugin.finish(app);
        }
    }

    pub(crate) fn cleanup(&mut self, app: &mut Application) {
        for plugin in self.plugins.iter_mut() {
            plugin.cleanup(app);
        }
    }
}


impl Default for PluginHandler {
    fn default() -> Self {
        Self::new()
    }
}
