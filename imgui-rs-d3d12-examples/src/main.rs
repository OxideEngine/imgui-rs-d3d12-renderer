mod common;

use common::*;
use simple_logger::SimpleLogger;
use std::error::Error;

const APP_NAME: &str = "test window";

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init()?;
    System::new(APP_NAME)?;

    Ok(())
}

