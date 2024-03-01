use events::EventHandler;
use std::sync::{Arc, Mutex};
use window::event_list::*;

pub fn init(resources: &mut legion::Resources, event_poll: Arc<Mutex<crate::RenderEvents>>) {
    let mut resize = resources.get_mut::<EventHandler<Resize>>().unwrap();

    resize.connect(move |size| {
        event_poll.lock().unwrap().resized = Some(size.clone());
        //println!("resized to : {} x {} y", size.width, size.height);
    })
}
