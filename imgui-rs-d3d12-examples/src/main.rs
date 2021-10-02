use std::error::Error;
use winit::{
    event_loop::EventLoop,
    window::{
        WindowBuilder,
        Window,
    },
    dpi::PhysicalSize,
};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

fn main() {
    create_window("Test Window");
}

fn create_window(title: &str) -> Result<(Window, EventLoop<()>), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
        .with_resizable(true)
        .build(&event_loop)?;

    Ok((window, event_loop))
}
