#![no_std]
#![no_main]

use defmt::{info, warn};
use defmt_rtt as _;
use panic_probe as _;

use embassy_rp::bind_interrupts;
use embassy_rp::i2c::{Config as I2cConfig, I2c, InterruptHandler as I2CInterruptHandler};
use embassy_rp::peripherals::I2C1;
use embedded_hal_async::i2c::I2c as _;
use embassy_time::{Timer, Duration};

mod irqs;
bind_interrupts!(struct Irqs {
    I2C1_IRQ => I2CInterruptHandler<I2C1>;
});

// Import
mod utils;
use utils::{creeate_pwm_config, update_pwm_config, Lane};

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let sda = peripherals.PIN_10;
    let scl = peripherals.PIN_11;

    let mut config = I2cConfig::default();
    config.frequency = 50000;
    let mut i2c = I2c::new_async(peripherals.I2C1, scl, sda, Irqs, config);

    let mut test_buf = [0u8; 1];
    //let mut found_buf:bool = false;

    // 0x08 -> 0x77
    let mut read_addr = 0x08u8;
    while read_addr <= 0x77 {
        match i2c.read(read_addr, &mut test_buf).await {
            Ok(_) => {
                info!("=> Found ok address 0x{:02x}", read_addr);
            },
            Err(_) => {
                //info!("!!! Not ok address 0x{:02x}", read_addr);
            }
        }
        read_addr += 1;
    }

    const BMP280_ADDR: u8 = 0x76; // BMP280 address

    // loop {

    // }
}