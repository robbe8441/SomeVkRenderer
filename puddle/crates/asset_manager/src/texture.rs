use std::{fmt::Debug, io::Write};

use bevy_ecs::{component::Component, system::Resource};

#[derive(Resource, Component)]
pub struct RawTexture {
    pub data: Vec<u8>,
    pub size: [u32; 3],
}

#[allow(unused)]
impl RawTexture {
    #[inline(always)]
    pub fn empty(size: impl Into<[u32; 3]>) -> Self {
        let size: [u32; 3] = size.into();
        let len = size.iter().product::<u32>() as usize;
        let data: Vec<u8> = vec![0; len];

        Self { data, size }
    }

    #[inline(always)]
    pub fn new(size: impl Into<[u32; 3]>, data: impl Into<Vec<u8>>) -> Self {
        let data: Vec<u8> = data.into();
        let size: [u32; 3] = size.into();

        if data.len() as u32 != size.iter().product() {
            panic!("data doesn't match expected size");
        }

        Self { data, size }
    }

    #[inline(always)]
    pub fn write_bytes(&mut self, bytes: &[u8], offset: usize) {
        self.data[offset..offset + bytes.len()]
            .as_mut()
            .write_all(bytes);
    }

    pub fn write_texture(&mut self, data: &RawTexture, offset: impl Into<[u32; 3]>) {
        let offset: [u32; 3] = offset.into();

        let offset_bytes =
            offset[0] + self.size[0] * offset[1] + self.size[0] * self.size[1] * offset[2];

        for layer in 0..data.size[2] {
            let layer_index = layer * data.size[0] * data.size[1];

            for row in 0..data.size[1] {
                let row_index = row * data.size[0];

                let write_start = (row_index + layer_index) as usize;
                let write_end = (write_start as u32 + data.size[0]) as usize;

                let to_write = &data.data[write_start..write_end];

                let write_offset = self.size[0] * row + self.size[0] * self.size[1] * layer;

                self.write_bytes(to_write, offset_bytes as usize + write_offset as usize);
            }
        }
    }
}

impl Debug for RawTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n")?;

        for y in 0..self.size[1] {
            let start = (y * self.size[0]) as usize;
            let end = ((y + 1) * self.size[0]) as usize;

            let slice = &self.data[start..end];
            f.write_str(format!("{:?}\n", slice).as_str())?;
        }

        Ok(())
    }
}
