use std::time::Instant;

use bevy_ecs::component::Component;
use bevy_ecs::system::{Commands, NonSendMut, Res, ResMut, Resource};
use noise::NoiseFn;
use puddle::asset_manager::Vertices;
use puddle::components::Transform;
use puddle::rendering::frontend::InstancingTransforms;
use puddle::*;

#[derive(Default, Resource, Component)]
struct TerrainTextures;

fn main() {
    let mut app = puddle::application::Application::default();

    app.add_plugin(window::WindowPlugin);
    app.add_plugin(rendering::RenderPlugin);
    app.add_plugin(asset_manager::AssetManagerPlugin);
    app.add_plugin(time::TimePlugin);

    app.add_systems(application::Update, (print_delta, update_cam));

    app.add_systems(application::PostStartup, load_model);

    app.world
        .insert_resource(AverageFPS(2000.0, Instant::now()));

    app.run();
}

fn load_model(mut commands: Commands) {
    let model = Vertices::cube();

    let mut data: Vec<u8> = Vec::new();

    let noise = noise::Worley::new(0);

    const CHUNK_SIZE: u32 = 10;

    const NOISE_SCALE: f64 = 5.0;

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let num = noise.get([
                    x as f64 / NOISE_SCALE,
                    y as f64 / NOISE_SCALE,
                    z as f64 / NOISE_SCALE,
                ]) > 0.0;
                data.push(num as u8 * 255);
            }
        }
    }

    let voxels = asset_manager::RawTexture::new([CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE], data);

    let instance_transforms = InstancingTransforms(vec![
        Transform::from_xyz(0.0, 0.0, 0.0),
    ]);

    commands.spawn((model, voxels, instance_transforms));
}



// fn update_cubes(mut query: Query<&mut Transform>, time: Res<time::Time>) {
//     for (i, mut transform) in query.iter_mut().enumerate() {
//         transform.translation.z = (time.startup.elapsed().as_secs_f32() + i as f32 / 2.0).sin();
//     }
// }

use puddle::rendering::frontend::types::Camera;

fn update_cam(mut cam: NonSendMut<Camera>, time: Res<time::Time>) {
    let t = time.startup.elapsed().as_secs_f32() / 3.0;
    use components::Transform;
    use glam::Vec3;

    let pos = Vec3::new(t.sin(), t.sin() / 4.0, t.cos()) * 2.0;
    cam.transform = Transform::from_translation(pos).looking_at(Vec3::ZERO, Vec3::Y)
}

#[derive(Resource)]
struct AverageFPS(f32, Instant);

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn print_delta(time: Res<time::Time>, mut avg: ResMut<AverageFPS>) {
    let fps = 1.0 / time.delta;

    avg.0 = lerp(avg.0, fps, 10.0 * time.delta);

    if avg.1.elapsed().as_secs() > 2 {
        println!("fps : {}", avg.0.floor());
        avg.1 = Instant::now();
    }
}
