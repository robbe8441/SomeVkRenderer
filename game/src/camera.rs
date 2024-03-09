use cgmath::{Vector3, Zero};
use puddle::rendering::Camera;
use puddle::window::winit::keyboard::KeyCode;
use puddle::input::*;


#[legion::system]
pub fn camera_controller(
    #[resource] camera : &mut Camera,
    #[resource] input : &ArcMut<Input>,
    #[resource] delta : &mut crate::DeltaTime,
) {
    let input = input.lock().unwrap();

    let look_v = camera.eye - camera.target;
    let right_v = look_v.cross(Vector3::unit_y());

    let mut movement = Vector3::zero();

    if input.key_pressed(KeyCode::KeyW) {
        movement -= look_v;
    }
    if input.key_pressed(KeyCode::KeyS) {
        movement += look_v;
    }
    if input.key_pressed(KeyCode::KeyA) {
        movement += right_v;
    }
    if input.key_pressed(KeyCode::KeyD) {
        movement -= right_v;
    }

    camera.eye += movement * delta.0 as f32 / 100.0;
}
