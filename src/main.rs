use puddle::*;

fn on_close(_: &()) {}

fn on_render(delta_time: &f64) {
}

fn on_resize(size: &puddle::PhysicalSize<u32>) {
    info!("new size : {:?}", size);
}

fn main() {
    puddle::init();
    let mut app = Application::new();

    app.window.set_title("Ultra cool Game");

    app.window.pre_render_event.connect(on_render);
    app.window.on_close_requested.connect(on_close);
    app.window.on_resize_event.connect(on_resize);

    app.renderer.attach_imgui(app.window.window.clone());
    app.run();
}
