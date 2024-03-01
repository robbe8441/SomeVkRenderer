use cgmath::{Vector3, Zero};
use legion::system;
use puddle::rendering::Camera;
use puddle::window::winit::keyboard::KeyCode;
use puddle::window::InputList;

#[system]
pub fn camera_controller(
    #[resource] camera: &mut Camera,
    #[resource] input: &InputList,
    #[resource] delta: &crate::DeltaTime,
) {
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
