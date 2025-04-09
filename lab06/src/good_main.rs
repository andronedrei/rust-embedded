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

const BMP280_ADDR: u8 = 0x76; // BMP280 address
const EEPROM_ADDR: u8 = 0x50; //AT24C256 E2PROM 
const EEPROM_STORAGE_ADDR: u16 = 0xACDC; //storage location 

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let sda = peripherals.PIN_10;
    let scl = peripherals.PIN_11;
    
    // Configure I2C with 100kHz clock
    let mut config = I2cConfig::default();
    config.frequency = 100_000; // 100kHz
    let mut i2c = I2c::new_async(peripherals.I2C1, scl, sda, Irqs, config);
    
    // I2C bus scan at startup
    let mut found = 0;
    let mut buf = [0u8; 1];
    
    for addr in 0x08u8..=0x77u8 {
        match i2c.read(addr, &mut buf).await {
            Ok(_) => {
                info!("Found I2C device at 0x{:02X}", addr);
                found += 1;
            },
            Err(_) => {
                // Ignore unreachable addresses
            },
        }
    }
    
    if found == 0 {
        warn!("No I2C devices found.");
    } else {
        info!("I2C scan complete. {} device(s) found.", found);
    }
    
    // BMP280 Configurationing 
    info!("Configuring BMP280...");
    if let Err(e) = i2c.write(BMP280_ADDR, &[0xF4, 0x43]).await { // Normal mode, temp x2
        warn!("BMP280 config failed: {:?}", e);
        return;
    }
    
    // Reading calibration data
    info!("Reading BMP280 calibration...");
    let mut calib_data = [0u8; 6];
    if i2c.write_read(BMP280_ADDR, &[0x88], &mut calib_data).await.is_err() {
        warn!("Failed to read calibration");
        return;
    }
    
    let dig_t1 = u16::from_le_bytes([calib_data[0], calib_data[1]]);
    let dig_t2 = i16::from_le_bytes([calib_data[2], calib_data[3]]);
    let dig_t3 = i16::from_le_bytes([calib_data[4], calib_data[5]]);
    info!("Calibration values: T1={} T2={} T3={}", dig_t1, dig_t2, dig_t3);
    
    // Try reading previous temperature from EEPROM
    let mut prev_temp = [0u8; 4];
    if i2c.write_read(EEPROM_ADDR, &EEPROM_STORAGE_ADDR.to_be_bytes(), &mut prev_temp).await.is_ok() {
        let temp = i32::from_be_bytes(prev_temp);
        info!("Previous temperature: {}.{:02}°C", temp/100, temp.abs()%100);
    } else {
        warn!("No previous temperature found in EEPROM");
    }
    
    info!("Starting temperature measurements...");
    
    loop {
        // Read temperature registers
        let mut temp_data = [0u8; 3];
        if i2c.write_read(BMP280_ADDR, &[0xFA], &mut temp_data).await.is_err() {
            warn!("Temperature read failed");
            Timer::after(Duration::from_secs(1)).await;
            continue;
        }
        
        // Calculate raw temperature (20-bit value)
        let raw_temp = ((temp_data[0] as u32) << 12) | ((temp_data[1] as u32) << 4) | ((temp_data[2] as u32) >> 4);
        // Temperature compensation (from datasheet)
        let var1 = (((raw_temp >> 3) as i32 - (dig_t1 as i32 * 2)) * dig_t2 as i32) >> 11;
        let var2 = (((((raw_temp >> 4) as i32 - dig_t1 as i32).pow(2)) >> 12) * dig_t3 as i32) >> 14;
        let t_fine = var1 + var2;
        let actual_temp = (t_fine * 5 + 128) >> 8; // in hundredths of °C
        
        info!(
            "Temperature: {}.{:02}°C (raw: {})",
            actual_temp / 100,
            actual_temp.abs() % 100,
            raw_temp
        );
        
        // Storing in EEPROM
        let temp_bytes = actual_temp.to_be_bytes();
        let mut write_buf = [0u8; 6];
        write_buf[0..2].copy_from_slice(&EEPROM_STORAGE_ADDR.to_be_bytes());
        write_buf[2..6].copy_from_slice(&temp_bytes);
        
        if let Err(e) = i2c.write(EEPROM_ADDR, &write_buf).await {
            warn!("EEPROM write failed: {:?}", e);
        } else {
            Timer::after(Duration::from_millis(5)).await; 
        }
        
        Timer::after(Duration::from_secs(1)).await;
    }
}