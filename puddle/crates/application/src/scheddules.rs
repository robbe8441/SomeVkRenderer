use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Scheddules {
    Startup,
    Update,
}

pub struct SchedduleHandler {
    list : HashMap<Scheddules, legion::systems::Builder>
}

impl SchedduleHandler {
    pub fn new() -> Self {
        Self {
            list : HashMap::new()
        }
    }

    pub fn remove(&mut self, schedule : Scheddules) -> Option<legion::systems::Builder> {
        self.list.remove(&schedule)
    }

    pub fn get_or_add(&mut self, schedule : Scheddules) -> *mut legion::systems::Builder {
        match self.list.get_mut(&schedule) {
            Some(s) => s,

            None => {
                let mut builder = legion::systems::Builder::default();
                self.list.insert(schedule.clone(), builder);
                self.list.get_mut(&schedule).unwrap()
            }
        }
    }

    pub fn add<T>(&mut self, schedule : Scheddules, system : T)
        where T : legion::systems::ParallelRunnable + 'static
    {
        let mut sys = self.get_or_add(schedule);
        unsafe { sys.as_mut().unwrap().add_system(system) };
    }


    pub fn add_non_parralel<T>(&mut self, schedule : Scheddules, system : T)
        where T : FnMut(&mut legion::World, &mut legion::Resources) + 'static
    {
        let mut sys = self.get_or_add(schedule);
        unsafe { sys.as_mut().unwrap().add_thread_local_fn(system) };
    }
}












