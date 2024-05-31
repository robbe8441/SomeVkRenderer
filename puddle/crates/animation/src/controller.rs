use super::animation::*;
use asset_manager::RawTexture;
use std::{sync::Arc, time::Instant};

use bevy_ecs::{
    component::Component,
    system::Resource,
};

#[derive(Component, Resource)]
pub struct AnimationController {
    pub playback_speed: f32,
    pub animation_playing: Arc<Animation>,

    start_time: Instant,
}

impl AnimationController {
    pub fn new(animation: Arc<Animation>) -> Self {
        animation.into()
    }

    pub fn get_frame(&self) -> &RawTexture {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let frame = (elapsed * self.animation_playing.fps) * self.playback_speed;

        let index = frame.floor() as usize % self.animation_playing.frames.len();

        &self.animation_playing.frames[index]
    }
}

impl From<Arc<Animation>> for AnimationController {
    fn from(input: Arc<Animation>) -> Self {
        Self {
            animation_playing: input.clone(),
            playback_speed: 1.0,
            start_time: Instant::now(),
        }
    }
}
