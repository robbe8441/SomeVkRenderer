use cgmath::{Vector3, Zero};
use puddle::window::winit::keyboard::KeyCode;

pub fn camera_controller(_world : &mut legion::World, resources : &mut legion::Resources) {
    let mut camera = resources.get_mut::<puddle::rendering::Camera>().unwrap();
    let input = resources.get_mut::<puddle::window::InputList>().unwrap();
    let delta = resources.get_mut::<crate::DeltaTime>().unwrap();

    let look_v = camera.eye - camera.target;
    let right_v = look_v.cross(Vector3::unit_y());

    let mut movement = Vector3::zero();

    if input.is_pressed(KeyCode::KeyW) {
        movement -= look_v;
    }
    if input.is_pressed(KeyCode::KeyS) {
        movement += look_v;
    }
    if input.is_pressed(KeyCode::KeyA) {
        movement += right_v;
    }
    if input.is_pressed(KeyCode::KeyD) {
        movement -= right_v;
    }

    camera.eye += movement * delta.0 as f32 / 100.0;
}
