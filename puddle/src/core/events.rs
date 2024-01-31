use log::trace;

pub struct EventDispatcher<T = ()> {
    handlers: Vec<Box<dyn Fn(&T)>>,
}

impl<T> EventDispatcher<T> {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn connect(&mut self, callback: impl Fn(&T) + 'static) {
        trace!("connected function to event");
        self.handlers.push(Box::new(callback));
    }

    pub fn fire(&self, event: &T) {
        for handler in &self.handlers {
            handler(event);
        }
    }
}
