use logger::warn;
use wgpu::Surface;

pub struct RenderContext<'a> {
    device : &'a wgpu::Device,
    queue : &'a wgpu::Queue,
    pub command_encoder : wgpu::CommandEncoder,
    pub frame : wgpu::SurfaceTexture,
    pub view : wgpu::TextureView,
}

impl <'a> RenderContext<'a> {
    pub fn new(device : &'a wgpu::Device, queue: &'a wgpu::Queue, surface: &'a Surface) -> Option<Self> {

        let frame = match surface.get_current_texture() {
            Ok(frame) => frame,
            Err(e) => {
                warn!("dropped frame: {e:?}");
                return None;
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label : None });

        Some(Self {
            device : device,
            queue : queue,
            command_encoder,
            frame,
            view,
        })
    }
    pub fn draw(self) {
        self.queue.submit(Some(self.command_encoder.finish()));
        self.frame.present();
    }
}