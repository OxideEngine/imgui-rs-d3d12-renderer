use std::error::Error;
use winit::{
    event_loop::EventLoop,
    window::{
        WindowBuilder,
        Window,
    },
    dpi::PhysicalSize,
};
use simple_logger::SimpleLogger;
use log;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

fn main() {
    SimpleLogger::new().init().unwrap();
    let (_window, _event_loop) = create_window("Test Window").unwrap();
}

fn create_window(title: &str) -> Result<(Window, EventLoop<()>), Box<dyn Error>> {
    log::debug!("Creating window and event loop");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
        .with_resizable(true)
        .build(&event_loop)?;

    Ok((window, event_loop))
}
