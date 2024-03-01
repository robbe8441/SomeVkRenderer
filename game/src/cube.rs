use puddle::rendering::Vertex;

pub fn get_cube() -> (Vec<Vertex>, Vec<u16>) {
    let vertecies = vec![
        Vertex {
            position: [-0.5, -0.5, -0.5],
            uv: [0.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, -0.5],
            uv: [0.5, 0.5],
        },
        Vertex {
            position: [0.5, 0.5, -0.5],
            uv: [1.0, 0.0],
        },
        Vertex {
            position: [-0.5, 0.5, -0.5],
            uv: [0.5, 0.5],
        },
        Vertex {
            position: [-0.5, 0.5, 0.5],
            uv: [0.0, 1.0],
        },
        Vertex {
            position: [0.5, 0.5, 0.5],
            uv: [0.0, 1.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.5],
            uv: [1.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.5],
            uv: [1.0, 1.0],
        },
    ];



    let indicies = vec![
        0, 2, 3, 2, 0, 1, 3, 2, 5, 5, 4, 3, 7, 5, 6, 5, 7, 4, 0, 6, 1, 0, 7, 6, 1, 6, 5, 5, 2, 1,
        4, 7, 0, 0, 3, 4,
    ];
    (vertecies, indicies)
}

pub fn get_data() -> (Vec<Vertex>, Vec<u16>) {
    let vertecies: Vec<Vertex> = [
        Vertex {
            position: [-0.0868241, 0.49240386, 0.0],
            uv: [0.4131759, 0.99240386],
        },
        Vertex {
            position: [-0.49513406, 0.06958647, 0.0],
            uv: [0.0048659444, 0.56958647],
        },
        Vertex {
            position: [-0.21918549, -0.44939706, 0.0],
            uv: [0.28081453, 0.05060294],
        },
        Vertex {
            position: [0.35966998, -0.3473291, 0.0],
            uv: [0.85967, 0.1526709],
        },
        Vertex {
            position: [0.44147372, 0.2347359, 0.0],
            uv: [0.9414737, 0.7347359],
        },
    ]
    .to_vec();

    //let indecies : Vec<u16> = vec![ 0, 2, 3, 2, 0, 1, 3, 2, 5, 5, 4, 3, 7, 5, 6, 5, 7, 4, 0, 6, 1, 0, 7, 6, 1, 6, 5, 5, 2, 1, 4, 7, 0, 0, 3, 4];

    let indecies: Vec<u16> = vec![0, 1, 4, 1, 2, 4, 2, 3, 4];

    (vertecies, indecies)
}
