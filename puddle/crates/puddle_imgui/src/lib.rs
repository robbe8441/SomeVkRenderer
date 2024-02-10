use application::DeltaTime;
use legion::system;
use log::{error, info};
use renderer::{PuddleRenderer, RenderContext};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use wgpu::naga::RelationalFunction::Any;
use window::PuddleWindow;
use winit::window::Window;

mod imgui_winit_support;
mod wgpu_imgui;

pub struct ImGuiPlugin;

struct ImGuiRenderer {
    renderer: wgpu_imgui::Renderer,
    platform: imgui_winit_support::WinitPlatform,
    imgui: imgui::Context,
    window: PuddleWindow,
}

static mut IMGUI_RENDERER: Option<ImGuiRenderer> = None;

impl application::Plugin for ImGuiPlugin {
    fn finish(&mut self, app: &mut application::Application) {
        let mut imgui = imgui::Context::create();
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);

        let window = app.resources.get::<PuddleWindow>().unwrap();
        info!("requesting renderer");
        let mut puddle_renderer = app.resources.get_mut::<renderer::PuddleRenderer>().unwrap();

        platform.attach_window(
            imgui.io_mut(),
            window.get_cloned(),
            imgui_winit_support::HiDpiMode::Default,
        );

        imgui.set_ini_filename(None);

        let hidpi_factor = window.get().scale_factor();
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
            texture_format: puddle_renderer.surface_config.format,
            ..Default::default()
        };

        let imgui_renderer = wgpu_imgui::Renderer::new(
            &mut imgui,
            &puddle_renderer.device,
            &puddle_renderer.queue,
            renderer_config,
        );

        let renderer = ImGuiRenderer {
            imgui,
            platform,
            renderer: imgui_renderer,
            window: window.clone(),
        };

        error!("adding draw pass");
        puddle_renderer.add_renderpass(draw);
        app.on_event.connect(handle_event);

        unsafe { IMGUI_RENDERER = Some(renderer) };
    }
}

fn handle_event(event : &winit::event::Event<()>) {
    let imgui = unsafe {IMGUI_RENDERER.take()};
    if let Some(mut imgui) = imgui {
        imgui.platform.handle_event(imgui.imgui.io_mut(), imgui.window.get_cloned() ,event);
        unsafe {IMGUI_RENDERER = Some(imgui)};
    }
}

fn draw(renderer: &mut PuddleRenderer, context: &mut RenderContext) {
    let mut imgui_renderer = unsafe { IMGUI_RENDERER.take() }.unwrap();
    let window = &imgui_renderer.window;
    imgui_renderer
        .platform
        .prepare_frame(imgui_renderer.imgui.io_mut(), window.get_cloned())
        .expect("failed to prepeare frame");
    let ui = imgui_renderer.imgui.frame();

    imgui_renderer.platform.prepare_render(ui, window.get());

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
                ui.text(format!("Frametime: {}", 1));
            });

        ui.show_demo_window(&mut true);
    }

    let mut rpass = context.begin_render_pass(wgpu::RenderPassDescriptor {
        label: Some("Imgui RenderPass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &renderer.view.as_ref().unwrap(),
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
    });
    imgui_renderer
        .renderer
        .render(
            imgui_renderer.imgui.render(),
            &renderer.queue,
            &renderer.device,
            &mut rpass,
        )
        .expect("Rendering failed");

    drop(rpass);

    unsafe { IMGUI_RENDERER = Some(imgui_renderer) };
}
