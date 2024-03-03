use std::collections::HashMap;

pub struct MouseDelta {pub x:f32, pub y:f32}


pub struct InputList(pub HashMap<winit::keyboard::PhysicalKey, bool>, pub legion::Resources);

impl InputList {
    pub fn is_pressed(&self, key : winit::keyboard::KeyCode) -> bool {
        let phis = winit::keyboard::PhysicalKey::Code(key);

        let val = self.0.get(&phis);
        if let Some(val) = val {
            return val.clone();
        }
        false
    }

    pub fn get_mouse_delta(&mut self) -> (f32, f32) {
        let delta = self.1.get_or_insert(MouseDelta {x:0.0, y:0.0});
        (delta.x, delta.y)
    }
}
