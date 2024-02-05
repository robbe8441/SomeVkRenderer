use std::fmt::format;
use std::sync::Arc;
use std::time::Instant;

use rendering::wgpu;
use window::winit;
mod wgpu_imgui;
mod imgui_winit_support;

pub struct PuddleImGuiRenderer {
    pub imgui: wgpu_imgui::Context,
    renderer: wgpu_imgui::Renderer,
    pub platform: imgui_winit_support::WinitPlatform,
    time : Instant
}

impl PuddleImGuiRenderer {
    pub fn draw_puddle_imgui(
        &mut self,
        device: &rendering::wgpu::Device,
        queue: &wgpu::Queue,
        window: Arc<winit::window::Window>,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
    ) {
        self.platform.prepare_frame(self.imgui.io_mut(), &window);
        let ui = self.imgui.frame();

        self.platform.prepare_render(ui, &window);

        {
            let window = ui.window("Hello world");
            window
                .size([300.0, 100.0], imgui::Condition::FirstUseEver)
                .build(|| {
                    ui.text("Hello world!");
                    ui.text("This...is...imgui-rs on WGPU!");
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });

            let window = ui.window("Hello too");
            window
                .size([400.0, 200.0], imgui::Condition::FirstUseEver)
                .position([400.0, 200.0], imgui::Condition::FirstUseEver)
                .build(|| {
                    ui.text(format!("Frametime: {}", self.time.elapsed().as_secs_f32()));
                    ui.text(format!("FPS : {}", (1.0 / self.time.elapsed().as_secs_f64()).floor() ));
                });

            self.time = Instant::now();

            ui.show_demo_window(&mut true);
        }

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: rendering::wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: rendering::wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        self.renderer
            .render(self.imgui.render(), &queue, &device, &mut rpass)
            .expect("Rendering failed");
    }
}

impl render::Renderer {
    pub fn attach_imgui(&mut self, window: Arc<winit::window::Window>) {
        let mut imgui = imgui::Context::create();
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);

        platform.attach_window(
            imgui.io_mut(),
            &window,
            imgui_winit_support::HiDpiMode::Default,
        );

        imgui.set_ini_filename(None);

        let hidpi_factor = window.scale_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    oversample_h: 1,
                    pixel_snap_h: true,
                    size_pixels: font_size,
                    ..Default::default()
                }),
            }]);

        let renderer_config = puddle_imgui::RendererConfig {
            texture_format: self.surface_desc.format,
            ..Default::default()
        };

        let renderer =
            wgpu_imgui::Renderer::new(&mut imgui, &self.device, &self.queue, renderer_config);

        self.imgui_render = Some(PuddleImGuiRenderer {
            imgui,
            renderer,
            platform,
            time : Instant::now(),
        });
    }
}
