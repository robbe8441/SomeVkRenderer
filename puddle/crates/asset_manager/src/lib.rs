mod load;
mod model;

use application::Update;

pub struct AssetManagerPlugin;

pub use model::{Vertices, ModelBundle};
pub use load::model_from_string;

impl application::Plugin for AssetManagerPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        app.add_systems(Update, model::load_vertex_buffers);
    }
}
