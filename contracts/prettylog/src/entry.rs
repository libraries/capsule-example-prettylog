use crate::error::Error;
use crate::logger;
use core::result::Result;

pub fn main() -> Result<(), Error> {
    logger::init().unwrap();
    log::info!("Hello World!");
    Ok(())
}
