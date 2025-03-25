// +---------------------------------------------------------------------------+
// |                             PM/MA lab skel                                |
// +---------------------------------------------------------------------------+

//! By default, this app prints a "Hello world" message with `defmt`.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_futures::yield_now;
use embassy_rp::{adc::Channel, gpio::{AnyPin, Level, Output, Pin, Pull}, peripherals};
use embassy_time::{Duration, Instant, Timer};
use {defmt_rtt as _, panic_probe as _};
// Use the logging macros provided by defmt.
use defmt::*;

// Import interrupts definition module
mod irqs;

//mod utils;

#[embassy_executor::task(pool_size = 2)]
async fn blink_led(pin: AnyPin, time: u64) {
    let half_time = time / 2;
    let mut led = Output::new(pin, Level::Low);

    loop {
        led.toggle();
        let start_time = Instant::now();
        
        while start_time.elapsed().as_millis() < half_time {
            yield_now().await;
        }
    }
}

//embasyy



#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let peripherals = embassy_rp::init(Default::default());

    info!("Hello world!");

    spawner.spawn(blink_led(peripherals.PIN_2.degrade(), 1000)).unwrap();
    spawner.spawn(blink_led(peripherals.PIN_3.degrade(), 1000)).unwrap();

    // loop {
        
    // }
}