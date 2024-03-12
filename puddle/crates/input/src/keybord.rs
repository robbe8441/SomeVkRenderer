#![allow(unused, dead_code)]

use crate::Input;
use std::collections::HashMap;




type KeybordKey = window::winit::keyboard::KeyCode;

struct Keybord {
    keys : HashMap<KeybordKey, bool>
}


impl Input<KeybordKey> for Keybord {
    fn press(&mut self, button: KeybordKey) {
        self.keys.insert(button, true);
    }

    fn is_pressed(&self, button: KeybordKey) -> bool {
        // return if the key exists and is pressed
        self.keys.get(&button).is_some_and(|x|*x)
    }
}





