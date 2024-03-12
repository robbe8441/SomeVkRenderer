use puddle::*;
use application::geese::*;



struct ListenToInput;

impl GeeseSystem for ListenToInput {
    const EVENT_HANDLERS: EventHandlers<Self> = event_handlers()
        .with(Self::print);

    fn new(_: GeeseContextHandle<Self>) -> Self {
        Self
    }
}

impl ListenToInput {
    fn print(&mut self, key:&window::events::KeybordInput) {
        dbg!(key);
    }
}


fn main() {
    let mut app = application::Application::new();

    app
        .add_event_listener::<ListenToInput>()
        .add_plugin(window::WindowPlugin);

    app.run();
}
