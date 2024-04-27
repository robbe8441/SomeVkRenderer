use super::*;

#[system]
pub fn update_cam(
    #[resource] renderer: &rendering::Renderer,
    #[resource] camera: &mut Camera,
    #[state] time: &Instant,
) {
    let r = 0.6;
    let t = time.elapsed().as_secs_f32() / 4.0;
    let x = (t * 2.0).cos() * r;
    let z = (t * 2.0).sin() * r;

    camera.data.eye = [x, 0.3, z].into();

    camera.camera_uniform.view_proj = camera.data.build_view_projection_matrix().into();
    renderer.update_buffer(&mut camera.uniform_buffer, &[camera.camera_uniform]);
}
