use std::sync::{Arc, Mutex};

use cgmath::{Vector3, Zero};
use legion::systems::CommandBuffer;
use puddle::rendering::Camera;
use puddle::window::winit::keyboard::KeyCode;

pub struct CurrentMousePos(f32, f32);

use puddle::input::*;

#[legion::system]
pub fn setup_cam(#[resource] input: &ArcMut<Input>, commands: &mut CommandBuffer) {
    let mut input = input.lock().unwrap();
    let pos = Arc::new(Mutex::new(CurrentMousePos(0.0, 0.0)));

    let pos_arc = pos.clone();

    input.event_handler.connect(move |event| {
        if let InputEvents::MouseDelta(x, y) = event {
            let mut pos = pos_arc.lock().unwrap();
            pos.0 += *x as f32 / 100.0;
            pos.1 -= *y as f32 / 100.0;
        }
    });

    let ent = commands.push(());
    commands.add_component(ent, pos);
}

#[legion::system(for_each)]
pub fn camera_controller(
    #[resource] input: &ArcMut<Input>,
    #[resource] camera: &mut Camera,
    #[resource] delta_time : &crate::DeltaTime,
    #[resource] paused : &mut crate::PlaybackPuased,
    mouse_pos: &ArcMut<CurrentMousePos>,
) {
    let input = input.lock().unwrap();
    let mouse_pos = mouse_pos.lock().unwrap();

    println!("{}", mouse_pos.1);

    let x = mouse_pos.1.sin() * mouse_pos.0.cos();
    let y = -mouse_pos.1.cos();
    let z = mouse_pos.1.sin() * mouse_pos.0.sin();

    let look_v = Vector3::new(x as f32, y as f32, z as f32);
    let up_v = Vector3::unit_y();
    let right_v = look_v.cross(up_v);

    let mut movement = Vector3::zero();

    paused.0 = input.key_pressed(KeyCode::KeyQ);

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
    if input.key_pressed(KeyCode::Space) {
        movement += up_v;
    }
    if input.key_pressed(KeyCode::ControlLeft) {
        movement -= up_v;
    }

    camera.eye += movement * delta_time.0 as f32 * 200.0;
    camera.target = camera.eye - Vector3::new(x as f32, y as f32, z as f32);
}
