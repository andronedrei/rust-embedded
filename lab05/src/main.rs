#![no_std]
#![no_main]
#![allow(unused_imports)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
use defmt::*;
use embassy_rp::gpio::{Output, Level};
use embassy_rp::spi::{self, Spi};
use embedded_hal_1::spi::SpiDevice as _;
const WHO_AM_I: u8 = 0x75; // in decimal, 117 (0x75)
const ACCEL_CONFIG: u8 = 0x1C; // in decimal, 28 (0x1C)
const GYRO_CONFIG: u8 = 0x1B; // in decimal, 27 (0x1B)
const ACCEL_XOUT_H: u8 = 0x43; // in decimal, 59 (0x3B)

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    
    let mut config = spi::Config::default();
    config.frequency = 1_000_000;
    config.phase = spi::Phase::CaptureOnFirstTransition;
    config.polarity = spi::Polarity::IdleLow;
    
    let mosi = p.PIN_3; // MOSI (GPIO)
    let miso = p.PIN_4; // MISO (GPIO)
    let clk = p.PIN_2;  // CLK (GPIO)
    
    let mut spi = Spi::new(p.SPI0, clk, mosi, miso, p.DMA_CH0, p.DMA_CH1, config);
    let mut cs = Output::new(p.PIN_5, Level::High);
    
    // Read WHO_AM_I register to check sensor communication
    cs.set_low();
    let tx_buf = [(1 << 7) | WHO_AM_I, 0x00];
    let mut rx_buf = [0u8; 2];
    spi.transfer(&mut rx_buf, &tx_buf).await.unwrap();
    cs.set_high();
    
    info!("WHO_AM_I: {}", rx_buf[1]);
    
    // Configure Accelerometer (±2g)
    cs.set_low();
    let _ = spi.write(&[ACCEL_CONFIG, 0b00000000]).await;
    cs.set_high();
    
    Timer::after(Duration::from_millis(10)).await;
    
    // Configure Gyroscope (±1000°/s)
    cs.set_low();
    let _ = spi.write(&[GYRO_CONFIG, 0b00010000]).await;
    cs.set_high();
    
    Timer::after(Duration::from_millis(10)).await;
    
    // Read and print sensor values in a loop
    loop {
        cs.set_low();
        let tx_buf = [(1 << 7) | ACCEL_XOUT_H];
        let mut rx_buf = [0u8; 15]; // 1 extra byte for response alignment
        spi.transfer(&mut rx_buf, &tx_buf).await.unwrap();
        cs.set_high();
        
        let accel_x_raw = i16::from_be_bytes([rx_buf[1], rx_buf[2]]);
        let accel_y_raw = i16::from_be_bytes([rx_buf[3], rx_buf[4]]);
        let accel_z_raw = i16::from_be_bytes([rx_buf[5], rx_buf[6]]);
        let gyro_x_raw = i16::from_be_bytes([rx_buf[9], rx_buf[10]]);
        let gyro_y_raw = i16::from_be_bytes([rx_buf[11], rx_buf[12]]);
        let gyro_z_raw = i16::from_be_bytes([rx_buf[13], rx_buf[14]]);
        
        // info!(
        //     "Accel: X={}, Y={}, Z={} | Gyro: X={}, Y={}, Z={}",
        //     accel_x_raw, accel_y_raw, accel_z_raw, gyro_x_raw, gyro_y_raw, gyro_z_raw
        // );
        
        let accel_x = (accel_x_raw as f32 / 16384.0) * 9.80665;
        let accel_y = (accel_y_raw as f32 / 16384.0) * 9.80665; 
        let accel_z = (accel_z_raw as f32 / 16384.0) * 9.80665;
        let gyro_x = gyro_x_raw as f32 / 32.8;
        let gyro_y = gyro_y_raw as f32 / 32.8;
        let gyro_z = gyro_z_raw as f32 / 32.8;
        
        info!(
            "Accel: X={}, Y={}, Z={} m/s²  |  Gyro: X={}, Y={}, Z={} °/s",
            accel_x, accel_y, accel_z , gyro_x, gyro_y, gyro_z
        );
        
        Timer::after(Duration::from_secs(1)).await;
    }
}
