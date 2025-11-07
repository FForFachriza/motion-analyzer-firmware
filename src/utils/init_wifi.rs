use anyhow::Context;
use esp_idf_svc::{
    eventloop::{EspEventLoop, System},
    hal::{delay::FreeRtos, prelude::Peripherals},
    nvs::{EspNvsPartition, NvsDefault},
    wifi::{BlockingWifi, EspWifi},
};

pub fn init_wifi(
    peripherals: Peripherals,
    sys_loop: EspEventLoop<System>,
    nvs: EspNvsPartition<NvsDefault>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    let ap_config = esp_idf_svc::wifi::Configuration::AccessPoint(
        esp_idf_svc::wifi::AccessPointConfiguration {
            ssid: "FForFachriza@rintohsaka".try_into().unwrap(),
            password: "".try_into().unwrap(),
            channel: 6,
            auth_method: esp_idf_svc::wifi::AuthMethod::None,
            max_connections: 4,
            ..Default::default()
        },
    );

    wifi.set_configuration(&ap_config)
        .context("Failed to Set Wifi Configuration")?;
    wifi.start().context("Failed to Start the Wifi")?;

    FreeRtos::delay_ms(500);

    log::info!("Access Point started!");
    log::info!("SSID: FForFachriza@rintohsaka");
    log::info!("Security: Open (no password)");
    log::info!("Channel: 6");

    Ok(())
}
