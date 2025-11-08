use anyhow::Context;
use esp_idf_svc::hal::{
    gpio::{Gpio21, Gpio22},
    i2c::{I2cConfig, I2cDriver, I2C0},
    units::Hertz,
};

pub fn init_i2c(
    i2c: I2C0,
    sda: Gpio21,
    scl: Gpio22,
) -> Result<I2cDriver<'static>, Box<dyn std::error::Error>> {
    let i2c_config = I2cConfig::new()
        .baudrate(Hertz::from(100000))
        .sda_enable_pullup(true)
        .scl_enable_pullup(true);

    let i2c = I2cDriver::new(i2c, sda, scl, &i2c_config).context("I2C Init Failed")?;

    log::info!("I2C initialized on GPIO21 (SDA) and GPIO22 (SCL)");

    Ok(i2c)
}
