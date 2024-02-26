use application::log::error;
use std::sync::Arc;

pub(crate) struct RenderContext {
    pub view: Arc<wgpu::TextureView>,
    pub frame: wgpu::SurfaceTexture,
    pub depth_texture: texture::Texture,
    pub command_encoder: wgpu::CommandEncoder,
}

impl RenderContext {
    pub fn new(
        surface: &wgpu::Surface,
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> Option<Self> {
        let frame = match surface.get_current_texture() {
            Ok(r) => r,
            Err(e) => {
                error!("frame dropped {}", e);
                return None;
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let command_encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        Some(Self {
            view: Arc::new(view),
            depth_texture: texture::Texture::cretate_depth_texture(&device, &surface_config),
            command_encoder,
            frame,
        })
    }

    pub fn execute(mut self, queue: &mut wgpu::Queue) {
        queue.submit([self.command_encoder.finish()]);
        self.frame.present();
    }
}
