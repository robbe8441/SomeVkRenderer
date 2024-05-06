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
pub struct Startup;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Update;


