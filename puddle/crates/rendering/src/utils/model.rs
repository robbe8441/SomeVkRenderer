use super::{material::Material, InstanceRaw};
use crate::{utils::ModelMatrix, Buffer, Renderer, Vertex};
use application::log::error;
use cgmath::{AbsDiffEq, Matrix4, Point3, Vector3, Zero};

use std::{
    collections::HashMap,
    fs,
    io::Read,
    path::Path,
    str::FromStr,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct MeshAsset {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub material: Arc<Material>,
    pub instance_buffer: Buffer,
    pub instances: Arc<Mutex<HashMap<u64, ModelMatrix>>>,
}

impl MeshAsset {
    pub fn reload_instance_buffer(&mut self, renderer: &Renderer) {
        let instances = self.instances.lock().and_then(|instances| {
            let raw_data: Vec<_> = instances
                .iter()
                .map(|(_, matrix)| matrix.to_raw())
                .collect();

            renderer.update_buffer(&mut self.instance_buffer, &raw_data);

            Ok(())
        });
    }
}

#[derive(Clone)]
pub struct Model {
    pub asset: MeshAsset,
    pub model_matrix: cgmath::Matrix4<f32>,
}

impl MeshAsset {
    pub fn create_model(&mut self) -> Model {
        Model {
            asset: self.clone(),
            model_matrix: cgmath::Matrix4::look_to_rh(
                Point3::new(0.0, 0.0, 0.0),
                Vector3::unit_z(),
                Vector3::unit_y(),
            ),
        }
    }
}
