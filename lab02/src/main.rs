#![no_main]
#![no_std]

use embassy_executor::Spawner;
use defmt_rtt as _;
use panic_probe as _;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    defmt::info!("Device has started!");
    
    let mut pin= Output::new(peripherals.PIN_2, Level::Low);

    loop {
        pin.set_high();
        Timer::after_millis(150).await;    
        pin.set_low();
        Timer::after_millis(150).await; 
    }
}