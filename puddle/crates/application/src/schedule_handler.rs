use std::collections::HashMap;

#[allow(unused)]
use crate::{Application, Plugin};

#[allow(unused)]
use bevy_ecs::{
    schedule::{ExecutorKind, InternedScheduleLabel, Schedule, ScheduleLabel, Schedules},
    system::{Local, Resource},
    world::{Mut, World},
};

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PreStartup;


#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Startup;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PostStartup;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PreUpdate;


#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Update;


#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PostUpdate;




pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&mut self, app: &mut Application) {

        let schedule_order: Vec<InternedScheduleLabel> = vec![
            PreStartup.intern(),
            Startup.intern(),
            PostStartup.intern(),

            PreUpdate.intern(),
            Update.intern(),
            PostUpdate.intern(),
        ];

        for label in schedule_order {
            let schedule = Schedule::new(label);
            app.schedules.insert(schedule);
        }
    }
}
