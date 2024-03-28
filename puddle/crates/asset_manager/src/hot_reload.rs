use crate::{error, Model};
use legion::system;
use rendering::Renderer;
use std::{io::Read, time::Instant};

pub struct HotReloading {
    file_path: String,
    last_changed: usize,
}

impl HotReloading {
    pub fn new(path: String) -> Self {
        use filetime::FileTime;
        use std::fs;

        let meta = fs::metadata(&path).unwrap();
        let last_changed = FileTime::from_last_modification_time(&meta).seconds() as usize;

        Self {
            file_path: path,
            last_changed,
        }
    }
}

struct LastReloadCheck(Instant);

pub(crate) fn init(app: &mut application::Application) {
    app.resources.insert(LastReloadCheck(Instant::now()));

    app.scheddules
        .add_non_parralel(application::Scheddules::Update, |world, resources| {

            let mut last_check = resources.get_mut::<LastReloadCheck>().unwrap();
            if last_check.0.elapsed().as_secs_f32() < 5.0 {
                return;
            }

            last_check.0 = Instant::now();

            let renderer = resources.get::<Renderer>().unwrap();

            use legion::*;
            let mut query = <(&mut Model, &mut HotReloading)>::query();

            for (model, hot_reload) in query.iter_mut(world) {
                use filetime::FileTime;
                use std::fs;
                use std::io::Result;

                let meta = fs::metadata(&hot_reload.file_path)
                    .inspect_err(|e| error!("failed to reload file : {}", e));
                if meta.is_err() {
                    return;
                }

                let last_changed =
                    FileTime::from_last_modification_time(&meta.unwrap()).seconds() as usize;
                println!(
                    "checking for reload : file {},  original {},  new : {}",
                    hot_reload.file_path, hot_reload.last_changed, last_changed,
                );

                if hot_reload.last_changed < last_changed {
                    let mut file = std::fs::File::open(&hot_reload.file_path).unwrap();

                    let mut str = String::new();
                    file.read_to_string(&mut str).unwrap();

                    let mesh = crate::model_from_string(&str);
                    *model = mesh.build(&renderer, model.material.clone());
                }
            }
        });
}
