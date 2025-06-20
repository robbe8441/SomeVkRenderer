use image::EncodableLayout;
use std::time::Instant;

use crate::{view::Chunktexture, PlaybackPuased};
use image::GenericImageView;
use legion::system;
use puddle::{application::log::warn, rendering::wgpu};

#[system(for_each)]
pub fn bad_apple(
    chunk: &Chunktexture,
    #[resource] paused: &PlaybackPuased,
    #[resource] renderer: &puddle::rendering::Renderer,
    #[state] frame: &Instant,
) {
    if paused.0 {
        return;
    }

    let frame = (frame.elapsed().as_secs_f64() * 60.0).floor() as i32;

    let image_path = format!("frames/{}.png", frame);

    let image = match image::open(image_path.clone()) {
        Ok(r) => r,
        Err(e) => {
            warn!("cant load image {}  because : {}", image_path, e);
            return;
        }
    };
    let size = image.dimensions();

    renderer.queue.write_texture(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &chunk.texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &image.to_luma8().as_bytes(),
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(size.0),
            rows_per_image: Some(size.1),
        },
        wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        },
    );
}
