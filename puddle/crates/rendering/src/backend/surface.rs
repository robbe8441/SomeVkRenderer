use std::sync::Arc;

use bevy_ecs::system::{Commands, Res, Resource};
use vulkano::swapchain::Surface;
use window::Window;

use super::instance::RenderInstance;

#[derive(Resource)]
pub struct RenderSurface(pub Arc<Surface>);

pub fn create_surface(mut commands: Commands, instance: Res<RenderInstance>, window: Res<Window>) {
    let surface = Surface::from_window(instance.0.clone(), window.0.clone()).unwrap();
    commands.insert_resource(RenderSurface(surface));
}
