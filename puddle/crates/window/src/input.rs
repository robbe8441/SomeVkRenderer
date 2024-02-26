use std::collections::HashMap;

pub struct InputList(pub HashMap<winit::keyboard::PhysicalKey, bool>);

impl InputList {
    pub fn is_pressed(&self, key : winit::keyboard::KeyCode) -> bool {
        let phis = winit::keyboard::PhysicalKey::Code(key);

        let val = self.0.get(&phis);
        if let Some(val) = val {
            return val.clone();
        }
        false
    }
}
