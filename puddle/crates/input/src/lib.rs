mod keybord;

pub trait Input<T> {

    // press a button
    // used to change the states 
    // or simulate input for tests

    fn press(&mut self, button: T);
    fn is_pressed(&self, button: T) -> bool;
}

pub struct InputPlugin;


impl application::Plugin for InputPlugin {

    fn build(&mut self, app: &mut application::Application) {

        



    }
}
