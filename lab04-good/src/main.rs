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
use embassy_rp::{adc::{Adc, InterruptHandler}, config, gpio::{AnyPin, Input, Level, Output, Pin, Pull}, pwm::{Pwm, SetDutyCycle}};
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


// #[embassy_executor::task(pool_size = 4)]
// async fn blink_led(mut led: Output<'static>, time: u64) {
//     let half_time = time / 2;

//     loop {
//         led.toggle();
//         let start_time = Instant::now();
        
//         while start_time.elapsed().as_millis() < half_time {
//             yield_now().await;
//         }
//     }
// }

#[embassy_executor::task(pool_size = 2)]
async fn adjust_inensity(mut sw1: Input<'static>, mut sw2: Input<'static>) {
    let mut intensity: u16 = 50;

    loop {
        let select_value = select(sw1.wait_for_rising_edge(), sw2.wait_for_rising_edge()).await;
        match select_value {
            First(_) => {
                intensity += 5;
            },
            Second(_) => {
                intensity -= 5;
            }
        }
        info!("{}", intensity);
        SIG.signal(intensity);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let peripherals = embassy_rp::init(Default::default());
    let mut adc = Adc::new(peripherals.ADC, Irqs, ConfigAdc::default());

    info!("Hello world!");

    let mut led_green = Output::new(peripherals.PIN_2, Level::Low);
    let mut led_yellow = Output::new(peripherals.PIN_3, Level::Low);
    let mut led_red = Output::new(peripherals.PIN_4, Level::Low);
    // let led4 = Output::new(peripherals.PIN_5, Level::Low);

    // spawner.spawn(blink_led(led1, 330)).unwrap();
    // spawner.spawn(blink_led(led2, 250)).unwrap();
    // spawner.spawn(blink_led(led3, 200)).unwrap();
    // spawner.spawn(blink_led(led4, 1000)).unwrap();

    let mut sw1: Input<'_> = Input::new(peripherals.PIN_6, Pull::None);
    let mut sw2: Input<'_> = Input::new(peripherals.PIN_7, Pull::None);

    // let mut config: ConfigPwm = creeate_pwm_config(50, Lane::LaneB);
    // let mut led_pwm = Pwm::new_output_b(
    //     peripherals.PWM_SLICE2, peripherals.PIN_5, config.clone() //GP5
    // );

    // let mut intensity: u16 = 50;

    // loop {
    //     let select_value = select(sw1.wait_for_rising_edge(), sw2.wait_for_rising_edge()).await;
    //     match select_value {
    //         First(_) => {
    //             intensity += 5;
    //         },
    //         Second(_) => {
    //             intensity -= 5;
    //         }
    //     }
    //     info!("{}", intensity);
    //     update_pwm_config(&mut config, intensity, Lane::LaneB);
    //     led_pwm.set_config(&config);
    // }

    //spawner.spawn(adjust_inensity(sw1, sw2)).unwrap();
    led_green.set_low();
    led_red.set_high();
    led_yellow.set_high();

    let mut state = 0;
    loop {
        if state == 0 {
            let (res1, res2) = join(sw1.wait_for_falling_edge(), sw2.wait_for_falling_edge()).await;
            Timer::after_millis(100).await;
            led_green.toggle();
            for _i  in 0..4 {
                led_yellow.toggle();
                Timer::after_millis(100).await
            }
            led_yellow.set_low();
        } else if state == 1 {
            let (res1, res2) = join(sw1.wait_for_falling_edge(), sw2.wait_for_falling_edge()).await;
            Timer::after_millis(100).await;
            led_yellow.toggle();
            led_red.toggle();
        } else if state == 2 {
            let (res1, res2) = join(sw1.wait_for_falling_edge(), sw2.wait_for_falling_edge()).await;
            Timer::after_millis(100).await;
            led_red.toggle();
            led_green.toggle();
        }

        state = (state + 1) % 3;
    }
}