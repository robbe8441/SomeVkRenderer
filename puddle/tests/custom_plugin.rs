use application::{Plugin, Startup, PostStartup};
use bevy_ecs::system::{Res, ResMut, Resource};
use puddle::*;

#[derive(Resource)]
struct TestResource(i32);

struct TestPlugin;
impl Plugin for TestPlugin {
    fn build(&mut self, app: &mut Application) {
        app.world.insert_resource(TestResource(10));
    }
}

fn change_resource(mut resource: ResMut<TestResource>) {
    resource.0 += 10;
}

fn read_resource(resource: Res<TestResource>) {
    assert_eq!(resource.0, 20);
}



#[test]
fn plugin() {
    let mut app = Application::new();
    app.add_plugin(TestPlugin);

    app.add_systems(Startup, read_resource);
    app.add_systems(PostStartup, read_resource);

    app.run();
}
