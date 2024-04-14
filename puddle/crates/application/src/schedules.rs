use std::{collections::HashMap, time::Duration};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Schedules {
    // fire on app startup
    Startup,
    // fire every frame
    Update,
    // fire every (...) secs
    UpdateEvery(Duration),
}

pub struct ScheduleHandler {
    pub list : HashMap<Schedules, legion::systems::Builder>
}

impl ScheduleHandler {
    pub fn new() -> Self {
        Self {
            list : HashMap::new()
        }
    }

    pub fn remove(&mut self, schedule : Schedules) -> Option<legion::systems::Builder> {
        self.list.remove(&schedule)
    }

    pub(crate) fn get_or_add(&mut self, schedule : Schedules) -> *mut legion::systems::Builder {
        match self.list.get_mut(&schedule) {
            Some(s) => s,

            None => {
                let mut builder = legion::systems::Builder::default();
                self.list.insert(schedule.clone(), builder);
                self.list.get_mut(&schedule).unwrap()
            }
        }
    }

    pub fn add<T>(&mut self, schedule : Schedules, system : T)
        where T : legion::systems::ParallelRunnable + 'static
    {
        let mut sys = self.get_or_add(schedule);
        unsafe { sys.as_mut().unwrap().add_system(system) };
    }


    pub fn add_non_parallel<T>(&mut self, schedule : Schedules, system : T)
        where T : FnMut(&mut legion::World, &mut legion::Resources) + 'static
    {
        let mut sys = self.get_or_add(schedule);
        unsafe { sys.as_mut().unwrap().add_thread_local_fn(system) };
    }
}












