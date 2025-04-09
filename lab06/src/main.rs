#![no_std]
#![no_main]

use defmt::info;
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

const BMP280_ADDR: u8 = 0x76;       // BMP280
const EEPROM_ADDR: u8 = 0x50;        // AT24C256 EEPROM
const EEPROM_STORAGE_ADDR: u16 = 0xACDC; // Unde dai store in EEPROM

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_rp::init(Default::default());
    let sda = p.PIN_10;
    let scl = p.PIN_11;

    let mut config = I2cConfig::default();
    config.frequency = 100_000;
    let mut i2c = I2c::new_async(p.I2C1, scl, sda, Irqs, config);

    if i2c.write(BMP280_ADDR, &[0xF4, 0x43]).await.is_err() {
        info!("BMP280 configuration failed");
        return;
    }

    let mut calib = [0u8; 6];
    if i2c.write_read(BMP280_ADDR, &[0x88], &mut calib).await.is_err() {
        info!("BMP280 calibration read failed");
        return;
    }
    let dig_t1 = u16::from_le_bytes([calib[0], calib[1]]);
    let dig_t2 = i16::from_le_bytes([calib[2], calib[3]]);
    let dig_t3 = i16::from_le_bytes([calib[4], calib[5]]);
    info!("Cal: T1={} T2={} T3={}", dig_t1, dig_t2, dig_t3);

    // Read previously stored temperature from EEPROM.
    let mut prev_temp = [0u8; 4];
    if i2c.write_read(EEPROM_ADDR, &EEPROM_STORAGE_ADDR.to_be_bytes(), &mut prev_temp)
        .await
        .is_ok()
    {
        let temp = i32::from_be_bytes(prev_temp);
        info!("Stored temp: {}.{:02}°C", temp / 100, temp.abs() % 100);
    } else {
        info!("No stored temperature in EEPROM");
    }

    // // Main measurement loop.
    // loop {
    //     let mut t_data = [0u8; 3];
    //     if i2c.write_read(BMP280_ADDR, &[0xFA], &mut t_data)
    //         .await
    //         .is_err()
    //     {
    //         info!("Temperature read failed");
    //         Timer::after(Duration::from_secs(1)).await;
    //         continue;
    //     }

    //     // Combine the bytes to get a 20-bit raw temperature value.
    //     let raw_temp = ((t_data[0] as u32) << 12)
    //         | ((t_data[1] as u32) << 4)
    //         | ((t_data[2] as u32) >> 4);

    //     // Compensate temperature as per datasheet.
    //     let var1 = (((raw_temp >> 3) as i32 - (dig_t1 as i32 * 2)) * (dig_t2 as i32)) >> 11;
    //     let var2 = ((((raw_temp >> 4) as i32 - dig_t1 as i32).pow(2) >> 12) * (dig_t3 as i32)) >> 14;
    //     let t_fine = var1 + var2;
    //     let temp = (t_fine * 5 + 128) >> 8; // temperature in hundredths of °C

    //     info!("Temp: {}.{:02}°C", temp / 100, temp.abs() % 100);

    //     // Write current temperature to EEPROM.
    //     let temp_bytes = temp.to_be_bytes();
    //     let addr_bytes = EEPROM_STORAGE_ADDR.to_be_bytes();
    //     let write_buf = [
    //         addr_bytes[0],
    //         addr_bytes[1],
    //         temp_bytes[0],
    //         temp_bytes[1],
    //         temp_bytes[2],
    //         temp_bytes[3],
    //     ];
    //     if i2c.write(EEPROM_ADDR, &write_buf).await.is_err() {
    //         info!("EEPROM write failed");
    //     } else {
    //         Timer::after(Duration::from_millis(5)).await;
    //     }

    //     Timer::after(Duration::from_secs(1)).await;
    // }
}
