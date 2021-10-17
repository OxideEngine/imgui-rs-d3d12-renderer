use std::error::Error;
use winit::{
    event_loop::EventLoop,
    window::{
        WindowBuilder,
        Window,
    },
    dpi::PhysicalSize,
};
use log;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

pub struct System {}

impl System {
    pub fn new(title: &str) -> Result<Self, Box<dyn Error>> {
        log::info!("Create Application");
        let (window, event_loop) = create_window(title)?;

        Ok(Self {})
    }
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
