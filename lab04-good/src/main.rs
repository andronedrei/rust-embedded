// +---------------------------------------------------------------------------+
// |                             PM/MA lab skel                                |
// +---------------------------------------------------------------------------+

//! By default, this app prints a "Hello world" message with `defmt`.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_futures::yield_now;
// use embassy_net::Config; // not useful
use embassy_time::{Duration, Instant, Timer};
use fixed::traits::ToFixed; // For 5 - servo
//use irqs::Irqs; // already defined
use {defmt_rtt as _, panic_probe as _};
use defmt::*; // Use the logging macros provided by defmt.

// use embassy_rp::{config, peripherals};
use embassy_rp::{adc::{Adc, Channel, InterruptHandler}, config, gpio::{AnyPin, Input, Level, Output, Pin, Pull}, pwm::{Pwm, SetDutyCycle}};
use embassy_rp::pwm::Config as ConfigPwm;
use embassy_rp::adc::Config as ConfigAdc; // ADC config
use embassy_rp::bind_interrupts;

// Import interrupts definition module
mod irqs;

// Import
//mod utils;\

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

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

// #[embassy_executor::task(pool_size = 1)]
// async fn decrease_inensity(mut 



#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let peripherals = embassy_rp::init(Default::default());
    let mut adc = Adc::new(peripherals.ADC, Irqs, ConfigAdc::default());

    info!("Hello world!");

    spawner.spawn(blink_led(peripherals.PIN_2.degrade(), 1000)).unwrap();
    spawner.spawn(blink_led(peripherals.PIN_3.degrade(), 1000)).unwrap();

    // loop {
        
    // }
}