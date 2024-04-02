use crate::{error, Model};
use rendering::utils::MeshAsset;

use filetime::FileTime;
use legion::system;
use rendering::Renderer;

use std::{
    fs,
    io::{Read, Result},
    path::Path,
    time::{Duration, Instant},
};

pub struct HotReloading {
    file_path: String,
    last_changed: i64,
}

impl HotReloading {
    /// creates a new reloader,
    /// used to check for file changes on assets
    /// last_changed is the last time the file changed
    ///
    /// to detect changes it compares the current last modification time with the saved one
    /// if it changed then it updates the asset
    ///
    /// Erors when the file cant be read
    pub fn new(file_path: &str) -> Result<Self> {
        let last_changed = Self::get_last_changed(file_path)?.seconds();

        Ok(Self {
            file_path: file_path.to_owned(),
            last_changed,
        })
    }

    pub fn from_model_builder(model : &crate::model::ModelBuilder) -> Self {
        Self::new(&model.file_path.clone().expect("no file path given")).unwrap()
    }

    /// get the last time a file changed
    pub fn get_last_changed(file_path: &str) -> Result<FileTime> {
        let meta =
            fs::metadata(file_path).inspect_err(|e| error!("failed to get change : {}", e))?;

        Ok(FileTime::from_last_modification_time(&meta))
    }

    /// Load the file as String,
    /// Erors when the file cant be read
    fn load_file(&self) -> Result<String> {
        let mut string = String::new();

        fs::OpenOptions::new()
            .read(true)
            .open(&self.file_path)
            .and_then(|mut file| file.read_to_string(&mut string))
            .inspect_err(|e| error!("failed to load file : {}", e));

        Ok(string)
    }
}

/// check for file changes
#[system(par_for_each)]
pub fn check_updates(
    asset: &mut MeshAsset,
    hot_reload: &mut HotReloading,
    #[resource] renderer: &Renderer,
) -> Result<()> {
    let last_changed = HotReloading::get_last_changed(hot_reload.file_path.as_str())?.seconds();

    if last_changed > hot_reload.last_changed {
        let updated_file = hot_reload.load_file()?;

        let new_data = super::model::model_from_string(&updated_file);

        renderer.update_buffer(&mut asset.vertex_buffer, &new_data.vertecies);
        renderer.update_buffer(&mut asset.index_buffer, &new_data.indecies);

        hot_reload.last_changed = last_changed;
    }

    Ok(())
}
