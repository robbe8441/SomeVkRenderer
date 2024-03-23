
use application::Plugin;

pub struct AssetManagerPlugin;

pub struct AssetManager;

impl Plugin for AssetManagerPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        app.resources.insert(AssetManager);
    }
}



impl AssetManager {
    pub fn create_material(&self) {
        println!("spawn");
    }
}





