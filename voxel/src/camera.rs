use cgmath::{Vector3, Zero};
use puddle::rendering::Camera;
use puddle::window::winit::keyboard::KeyCode;
use puddle::window::InputList;

struct CurrentMousePos(f32, f32);

pub fn camera_controller(_world: &mut legion::World, resouces: &mut legion::Resources) {
    resouces.get_or_insert(CurrentMousePos(0.0, 0.0));

    let mut input = resouces.get_mut::<InputList>().unwrap();
    let mut camera = resouces.get_mut::<Camera>().unwrap();
    let mut mouse_pos = resouces.get_mut::<CurrentMousePos>().unwrap();

    let delta = input.get_mouse_delta();

    let x = mouse_pos.1.sin() * mouse_pos.0.cos();
    let y = -mouse_pos.1.cos();
    let z = mouse_pos.1.sin() * mouse_pos.0.sin();

    mouse_pos.0 += delta.0 / 100.0;
    mouse_pos.1 += delta.1 / 100.0;

    let look_v = Vector3::new(x as f32, y as f32, z as f32);
    let up_v = Vector3::unit_y();
    let right_v = look_v.cross(up_v);

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
    if input.is_pressed(KeyCode::Space) {
        movement += up_v;
    }
    if input.is_pressed(KeyCode::ControlLeft) {
        movement -= up_v;
    }

    camera.eye += movement / 10.0;
    camera.target = camera.eye - Vector3::new(x as f32, y as f32, z as f32);
}
