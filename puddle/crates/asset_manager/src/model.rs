use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    query::Without,
    system::{Commands, Query},
};
use components::Transform;
use rendering::{
    backend::buffer::StandardMemoryAllocator,
    frontend::types::{Vertex3D, VertexBuffer},
};

#[derive(Component, Default, Debug)]
pub struct Vertices(pub Vec<Vertex3D>);

#[derive(Bundle, Default)]
pub struct ModelBundle {
    transform: Transform,
    vertices: Vertices,
}

use bevy_ecs::system::Res;
pub fn load_vertex_buffers(
    memory_allocator: Res<StandardMemoryAllocator>,
    model_query: Query<(Entity, &Vertices), Without<VertexBuffer>>,
    mut commands: Commands,
) {
    use rendering::frontend::types::VertexBuffer;

    for (entity, vertices) in model_query.iter() {
        let buffer = VertexBuffer::new(&memory_allocator, &vertices.0);
        dbg!("loaded buffer");
        commands.entity(entity).insert(buffer);
    }
}
