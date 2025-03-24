// +---------------------------------------------------------------------------+
// |                             PM/MA lab skel                                |
// +---------------------------------------------------------------------------+

//! By default, this app prints a "Hello world" message with `defmt`.

#![no_std]
#![no_main]

use core::ptr::read;

use embassy_executor::Spawner;
// use embassy_net::Config; // not useful
use embassy_time::{Duration, Timer};
use fixed::traits::ToFixed; // For 5 - servo
//use irqs::Irqs; // already defined
use {defmt_rtt as _, panic_probe as _};
use defmt::*; // Use the logging macros provided by defmt.

// use embassy_rp::{config, peripherals};
use embassy_rp::{adc::{Adc, Channel, InterruptHandler}, config, gpio::{Input, Pull}, pwm::{Pwm, SetDutyCycle}};
use embassy_rp::pwm::Config as ConfigPwm;
use embassy_rp::adc::Config as ConfigAdc; // ADC config
use embassy_rp::bind_interrupts;

// Import interrupts definition module
mod irqs;

// Import
mod utils;
use utils::{creeate_pwm_config, update_pwm_config, Lane};

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let peripherals = embassy_rp::init(Default::default());

    // Same for ADC (analog to DC)
    let mut adc = Adc::new(peripherals.ADC, Irqs, ConfigAdc::default());

    info!("Hello world!");
    // Test, comment later
    loop {
        info!("Hello\n");
        Timer::after_millis(200).await;
    }

    //let delay = Duration::from_secs(1);

    // * COMUN LA MAI MULTE EXERCITII
    // let mut config: ConfigPwm = creeate_pwm_config(25, Lane::LaneB);
    // let mut led_pwm = Pwm::new_output_b(
    //     peripherals.PWM_SLICE2, peripherals.PIN_5, config.clone() //GP5
    // );
    
    // * Exercitiul 1.1
    // loop {
    //     // Exercitiu 1.1
    //     led_pwm.set_config(&config);
    //     Timer::after_secs(1).await;
    // }

    // * Exercitiu 1.2
    // loop {
    //     let mut intensity: u16 = 0;

    //     for _i in 0..10 {
    //         update_pwm_config(&mut config, intensity, Lane::LaneB);
    //         led_pwm.set_config(&config);
    //         Timer::after_millis(250).await;
    //         intensity = (intensity + 10) % 100;
    //     }
    // }

    // * Exercitiu 2
    // let mut control_adc_pin = Channel::new_pin(peripherals.PIN_26, Pull::None); //GP26
    // const MAX_POTENTIOMETER_VALUE: u32 = 4095;

    // loop {
    //     let read_level = adc.read(&mut control_adc_pin).await.unwrap();
    //     info!("{}\n", read_level);
        
    //     update_pwm_config(&mut config, ((read_level as u32 * 100) / MAX_POTENTIOMETER_VALUE) as u16, Lane::LaneB);
    //     led_pwm.set_config(&config);

    //     Timer::after_millis(200).await;
    // }

    // * Exercitiul 3 (foloseste definitiile si la 4.2) // VA FOLOSI LEDURILE GP6, GP9, GP11, pe canalele 3 (A), 4 (B) si 5 (B)
    // let mut config_69: ConfigPwm = creeate_pwm_config(0, Lane::Both);
    // let mut config_11: ConfigPwm = creeate_pwm_config(0, Lane::LaneB);

    // // componente PWM led semafor
    // let mut component_red = Pwm::new_output_a(
    //     peripherals.PWM_SLICE3, peripherals.PIN_6, config_69.clone()
    // );
    // let mut component_green = Pwm::new_output_b(
    //     peripherals.PWM_SLICE4, peripherals.PIN_9, config_69.clone()
    // );
    // let mut component_blue = Pwm::new_output_b(
    //     peripherals.PWM_SLICE5, peripherals.PIN_11, config_11.clone()
    // );

    // let mut color_idx = 0;

    // // pin trigger
    // let mut pin_switch_color = Input::new(peripherals.PIN_15, Pull::None); //GP15

    // loop {
    //     pin_switch_color.wait_for_falling_edge().await;
    //     if color_idx == 0 {
    //         update_pwm_config(&mut config_69, 100, Lane::LaneA);
    //         update_pwm_config(&mut config_69, 0, Lane::LaneB);
    //         update_pwm_config(&mut config_11, 0, Lane::LaneB);
    //     } else if color_idx == 1 {
    //         update_pwm_config(&mut config_69, 100, Lane::Both);
    //         update_pwm_config(&mut config_11, 0, Lane::LaneB);
    //     } else if color_idx == 2 {
    //         update_pwm_config(&mut config_69, 0, Lane::Both);
    //         update_pwm_config(&mut config_11, 100, Lane::LaneB);
    //     } else {
    //         info!("Sth went wrong!\n");
    //     }
    //     component_red.set_config(&config_69);
    //     component_green.set_config(&config_69);
    //     component_blue.set_config(&config_11);

    //     color_idx = (color_idx + 1) % 3;
    //     pin_switch_color.wait_for_rising_edge().await;

    //     Timer::after_millis(200).await;
    // }

    // * Exercitiul 4.1
    // let mut photoresistor_pin = Channel::new_pin(peripherals.PIN_26, Pull::None); //GP26
    // const MAX_PHOTO_VALUE: u16 = 1600;
    // const MIN_PHOTO_VALUE: u16 = 400;
    // let treshold_step: u16 = (MAX_PHOTO_VALUE - MIN_PHOTO_VALUE) / 3;
    // let treshold_1 = MIN_PHOTO_VALUE + treshold_step;
    // let treshold_2: u16 = MIN_PHOTO_VALUE + 2 * treshold_step;

    // loop {
    //     let photo_read_level = adc.read(&mut photoresistor_pin).await.unwrap();
    //     info!("Nivel lumina: {}\n", photo_read_level);
    //     Timer::after_millis(100).await;

    // // // * Exercitiul 4.2
    //     if photo_read_level < treshold_1 {
    //         update_pwm_config(&mut config_69, 100, Lane::LaneA);
    //         update_pwm_config(&mut config_69, 0, Lane::LaneB);
    //         update_pwm_config(&mut config_11, 0, Lane::LaneB);
    //     } else if photo_read_level >= treshold_1 && photo_read_level < treshold_2 {
    //         update_pwm_config(&mut config_69, 100, Lane::Both);
    //         update_pwm_config(&mut config_11, 0, Lane::LaneB);
    //     } else if photo_read_level >= treshold_2 {
    //         update_pwm_config(&mut config_69, 0, Lane::Both);
    //         update_pwm_config(&mut config_11, 100, Lane::LaneB);
    //     } else {
    //         info!("Sth went wrong!\n");
    //     }
    //     component_red.set_config(&config_69);
    //     component_green.set_config(&config_69);
    //     component_blue.set_config(&config_11);

    //     Timer::after_millis(100).await;
    // }

    // * Exercitiul 5
    // // Configure PWM for servo control
    // let mut servo_config: ConfigPwm = Default::default();

    // // Set the calculated TOP value for 50 Hz PWM
    // servo_config.top = 0xB71A; 

    // // Set the clock divider to 64
    // servo_config.divider = 64_i32.to_fixed(); // Clock divider = 64

    // // Servo timing constants
    // const PERIOD_US: usize = 20_000; // 20 ms period for 50 Hz
    // const MIN_PULSE_US: usize = 500; // 0.5 ms pulse for 0 degrees
    // const MAX_PULSE_US: usize = 2500; // 2.5 ms pulse for 180 degrees

    // // Calculate the PWM compare values for minimum and maximum pulse widths
    // let min_pulse = (MIN_PULSE_US * servo_config.top as usize) / PERIOD_US;
    // let max_pulse = (MAX_PULSE_US * servo_config.top as usize) / PERIOD_US;

    // let mut servo = Pwm::new_output_a( // GP2
    //     peripherals.PWM_SLICE1,
    //     peripherals.PIN_2,
    //     servo_config.clone()
    // );


    // let rotation_time = Duration::from_millis(100);
    // loop {
    //     // servo_config.compare_a = max_pulse as u16;
    //     // servo.set_config(&servo_config);

    //     // Timer::after(rotation_time).await;
        
    //     // servo_config.compare_a = min_pulse as u16;
    //     // servo.set_config(&servo_config);

    //     // Timer::after(rotation_time).await;

    //     let read_level = adc.read(&mut control_adc_pin).await.unwrap();
    //     let pulse_adc_level = ((max_pulse as u32 - min_pulse as u32) * read_level as u32 / MAX_POTENTIOMETER_VALUE) as u16 + min_pulse as u16;

    //     servo_config.compare_a = pulse_adc_level;
    //     servo.set_config(&servo_config);

    //     Timer::after(rotation_time).await;
    // }
}
