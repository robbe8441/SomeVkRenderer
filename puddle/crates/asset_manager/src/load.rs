use std::str::FromStr;

use application::log::error;
use rendering::frontend::types::Vertex3D;

use crate::Vertices;

fn parse_numbers<T: Default + FromStr>(input: &str) -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    let mut res = vec![];

    for num in input.split_whitespace() {
        let parsed = match num.parse() {
            Ok(r) => r,
            Err(e) => {
                error!(
                    "Failed parsing number when loading model : {} number : {}",
                    e, num
                );
                continue;
            }
        };
        res.push(parsed);
    }
    res
}

pub fn model_from_string(input: &str) -> Vertices {
    let mut vertex_positions = Vec::new();
    let mut model_vertices = Vec::new();
    // let mut vertex_normals = Vec::new();
    // let mut vertex_uvs = Vec::new();

    for line in input.lines() {
        match &line[..2] {
            "v " => {
                let pos: [f32; 3] = parse_numbers(&line[2..])[0..3].try_into().unwrap();
                vertex_positions.push(pos);
            }
            // "vn" => {
            //     let pos: [f32; 3] = parse_numbers(&line[2..])[0..3].try_into().unwrap();
            //     vertex_normals.push(pos);
            // }
            // "vt" => {
            //     let pos: [f32; 2] = parse_numbers(&line[2..])[0..2].try_into().unwrap();
            //     vertex_uvs.push(pos);
            // }
            "f " => {
                let face_data = &line[2..].replace("/", " ");
                //println!("{:?} in line {}", face_data, i);
                let data: [usize; 9] = match parse_numbers(face_data)[0..9].try_into() {
                    Ok(r) => r,
                    Err(e) => {
                        error!("failed to laod face : {}", e);
                        continue;
                    }
                };

                for i in 0..3 {
                    let face_data = &data[i * 3..];
                    model_vertices.push(Vertex3D {
                        position: vertex_positions[face_data[0] - 1],
                        // uv_cords: vertex_uvs[face_data[1] - 1],
                        // normal: vertex_normals[face_data[2] - 1],
                    });
                }

                // model.indecies.extend((len as u32)..(len + 3) as u32);
            }
            _ => {}
        }
    }

    Vertices(model_vertices)
}
