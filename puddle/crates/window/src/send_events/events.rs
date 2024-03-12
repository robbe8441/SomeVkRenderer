
#[derive(Debug)]
pub struct KeybordInput {
    pub key: winit::keyboard::KeyCode,
    pub state: bool,
}


#[derive(Debug)]
pub struct ResizeWindow(pub winit::dpi::PhysicalSize<u32>);
