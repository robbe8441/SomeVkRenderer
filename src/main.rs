use std::fmt::Debug;

use puddle::*;
use puddle::legion::system;

#[system(for_each)]
fn for_each(plr: &Player) {
    dbg!(plr);
}

#[system]
fn print() {
    println!("hello world");
}

#[allow(dead_code)]
struct Location(f32, f32);

#[allow(dead_code)]
#[derive(Debug)]
struct Player {
    health : u32,
    name : String,
}

fn main() {
    let mut app = Application::new();

    app.world.extend(vec![
        (Player { health : 100, name: "robbe".to_owned()}, Location(10.0, 20.0)),
        (Player { health : 100, name: "steve".to_owned()}, Location(400.0, 30.0)),
    ]);

    

    app.add_plugin(WindowPlugin);
    app.add_plugin(DefaultRenderer);
    app.add_plugin(ImGuiPlugin);

    app.run();
}
