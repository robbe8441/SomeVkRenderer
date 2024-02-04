use std::sync::{Arc, Mutex};
use std::time::Instant;
use legion::*;

mod wgpu_imgui;
mod imgui_winit_support;
use application::wgpu;


pub struct PuddleImGuiRenderer {
    imgui:wgpu_imgui::Context,
    renderer: wgpu_imgui::Renderer,
    platform: imgui_winit_support::WinitPlatform,
    time : Instant
}


pub fn draw_imgui(puddle_imgui : &mut PuddleImGuiRenderer,renderer : &mut application::rendering::Renderer, window : &Arc<application::winit::window::Window>) {
    puddle_imgui.platform.prepare_frame(puddle_imgui.imgui.io_mut(), &window);
    let ui = puddle_imgui.imgui.frame();

    puddle_imgui.platform.prepare_render(ui, &window);

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
                ui.text(format!("Frametime: {}", puddle_imgui.time.elapsed().as_secs_f32()));
                ui.text(format!("FPS : {}", (1.0 / puddle_imgui.time.elapsed().as_secs_f64()).floor() ));
            });

        puddle_imgui.time = Instant::now();

        ui.show_demo_window(&mut true);
    }

    if renderer.command_encoder.is_none() || renderer.view.is_none() {
        return;
    }

    let mut rpass = renderer.command_encoder.as_mut().unwrap().begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &renderer.view.as_ref().unwrap(),
            resolve_target: None,
            ops: wgpu::Operations {
                load: application::wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }),
                store: application::wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
    });

    puddle_imgui.renderer
        .render(puddle_imgui.imgui.render(), &renderer.queue, &renderer.device, &mut rpass)
        .expect("Rendering failed");
}



pub fn attach_imgui(app : &mut application::Application) {

    let renderer = app.resources.get::<application::rendering::Renderer>().expect("pls first add renderer");

    let mut imgui = imgui::Context::create();
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);

    platform.attach_window(
        imgui.io_mut(),
        &app.window,
        imgui_winit_support::HiDpiMode::Default,
    );

    imgui.set_ini_filename(None);

    let hidpi_factor = app.window.scale_factor();
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

    let renderer_config = wgpu_imgui::RendererConfig {
        texture_format: renderer.surface_desc.format,
        ..Default::default()
    };

    let renderer = wgpu_imgui::Renderer::new(&mut imgui, &renderer.device, &renderer.queue, renderer_config);
    let time = Instant::now();

    let puddle_imgui = PuddleImGuiRenderer {
        imgui, renderer, platform, time
    };
}
