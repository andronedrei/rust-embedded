// +---------------------------------------------------------------------------+
// |                             PM/MA lab skel                                |
// +---------------------------------------------------------------------------+

//! By default, this app prints a "Hello world" message with `defmt`.

#![no_std]
#![no_main]

use core::cmp::{max, min};

use embassy_executor::Spawner;
use embassy_futures::{select::select, yield_now};
// use embassy_net::Config; // not useful
use embassy_time::{Duration, Instant, Timer};
use fixed::traits::ToFixed; // For 5 - servo
//use irqs::Irqs; // already defined
use {defmt_rtt as _, panic_probe as _};
use defmt::*; // Use the logging macros provided by defmt.

// use embassy_rp::{config, peripherals};
use embassy_rp::{adc::{Adc, InterruptHandler}, config, gpio::{AnyPin, Input, Level, Output, Pin, Pull}, pwm::{Pwm, SetDutyCycle}, spi::{self, Spi}};
use embassy_rp::pwm::Config as ConfigPwm;
use embassy_rp::adc::Config as ConfigAdc; // ADC config
use embassy_rp::bind_interrupts;

use embassy_sync::{blocking_mutex::raw::{CriticalSectionRawMutex, ThreadModeRawMutex}, channel::{Channel, Receiver, Sender}};
use embassy_sync::signal::Signal;
use embassy_futures::select::Either::Second;
use embassy_futures::select::Either::First;
use embassy_futures::join::join;


// Import interrupts definition module
mod irqs;

// Import
mod utils;
use utils::{creeate_pwm_config, update_pwm_config, Lane};


bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

pub enum LedControl {
    Increase,
    Decrease,
}

static CHANNEL: Channel<ThreadModeRawMutex, LedControl, 64> = Channel::new();
static SIG: Signal<CriticalSectionRawMutex, u16> = Signal::new();


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello world!");
    const REG_ADDR: u8 = 0x75;

    // Get a handle to the RP's peripherals.
    let peripherals = embassy_rp::init(Default::default());
    let mut adc = Adc::new(peripherals.ADC, Irqs, ConfigAdc::default());

    // intializare
    let mut config = spi::Config::default();
    config.frequency = 1_000_000;
    config.phase = spi::Phase::CaptureOnFirstTransition;
    config.polarity = spi::Polarity::IdleLow;

    let mosi = peripherals.PIN_3;
    let miso = peripherals.PIN_4;
    let clk = peripherals.PIN_2;

    let mut spi = Spi::new(peripherals.SPI0, clk, mosi, miso, peripherals.DMA_CH0, peripherals.DMA_CH1, config);
    let mut cs = Output::new(peripherals.PIN_5, Level::High);

    cs.set_low();
    let tx_buf = [(1 << 7) | REG_ADDR, 0x00]; // first value of buffer is the control byte, second is a *don't care* value
    let mut rx_buf = [0u8; 2]; // initial values that will be replaced by the received bytes
    spi.transfer(&mut rx_buf, &tx_buf).await.unwrap();
    cs.set_high();

    let register_value = rx_buf[1]; // the second byte in the buffer will be the received register value
    info!("{}", register_value);
    //117
    // loop {

    // }
}