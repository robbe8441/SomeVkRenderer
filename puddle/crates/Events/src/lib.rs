#![allow(unused, dead_code)]

pub struct EventHandler<T> {
    connections : Vec<Box<dyn Fn(&mut T)>>
}


impl <T>EventHandler<T> {
    pub fn new() -> Self {
        Self {
            connections : Vec::new()
        }
    }

    pub fn fire(&self, data : &mut T) {
        for func in self.connections.iter() {
            func(data);
        }
    }

    pub fn connect(&mut self, func : impl Fn(&mut T) + 'static) {
        self.connections.push(Box::new(func));
    }
}









