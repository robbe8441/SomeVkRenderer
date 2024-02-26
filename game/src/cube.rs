use puddle::rendering::Vertex;

pub fn get_data() -> (Vec<Vertex>, Vec<u16>) {
    let mut vertecies : Vec<Vertex> = vec![
        Vertex {
            position: [-1.0, -1.0, -1.0],
            color: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: [1.0, -1.0, -1.0],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [1.0, 1.0, -1.0],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [-1.0, 1.0, -1.0],
            color: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-1.0, 1.0, 1.0],
            color: [1.0, 1.0, 0.0],
        },
        Vertex {
            position: [1.0, 1.0, 1.0],
            color: [0.0, 1.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0, 1.0],
            color: [1.0, 1.0, 1.0],
        },
        Vertex {
            position: [-1.0, -1.0, 1.0],
            color: [1.0, 0.0, 1.0],
        },
    ];


    let indecies : Vec<u16> = vec![ 0, 2, 3, 2, 0, 1, 3, 2, 5, 5, 4, 3, 7, 5, 6, 5, 7, 4, 0, 6, 1, 0, 7, 6, 1, 6, 5, 5, 2, 1, 4, 7, 0, 0, 3, 4];

    (vertecies, indecies)
}


