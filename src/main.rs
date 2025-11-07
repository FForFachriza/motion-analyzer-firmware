mod utils;

use crate::utils::init_wifi::init_wifi;
use anyhow::Context;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop, hal::prelude::Peripherals, nvs::EspDefaultNvsPartition,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().context("Failed To Init Peripherals!")?;
    let sys_loop = EspSystemEventLoop::take().context("Failed to Init ESP Event Loop")?;
    let nvs = EspDefaultNvsPartition::take().context("Failed To Init NVS")?;

    init_wifi(peripherals, sys_loop, nvs).unwrap();

    log::info!("Hello, world!");

    Ok(())
}
