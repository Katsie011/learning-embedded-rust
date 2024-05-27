// #![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::{borrow::BorrowMut, default};

// Halt on panic
use panic_halt as _;

use cortex_m_rt::entry;

use stm32f0xx_hal as hal;

use crate::hal::{delay::Delay, gpio::*, pac::Peripherals, prelude::*};

use cortex_m::peripheral::Peripherals as c_m_Peripherals;

// For semihosted prints on the opencd terminal
use cortex_m_semihosting::hprintln;
// use panic_semihosting as _; // features = ["exit"]

fn flash_led(led: &mut hal::gpio::Pin<hal::gpio::Output<hal::gpio::PushPull>>, sleep_ms: u16) {
    // Running without interrupts:
    // Turn on LED

    // crude delay
    led.set_high().ok();
    for _i in 0..sleep_ms {
        continue;
    }

    // Wait a bit
    // Turn off LED
    led.set_low().ok();
    for _i in 0..sleep_ms {
        continue;
    }
}

fn test_leds(leds: &mut [hal::gpio::Pin<hal::gpio::Output<hal::gpio::PushPull>>]) {
    for led in leds {
        flash_led(led, 500);
        flash_led(led, 500);
    }
}

fn write_to_leds(number: u8, leds: &mut [hal::gpio::Pin<hal::gpio::Output<hal::gpio::PushPull>>]) {
    let mut num = number as u8;
    let power = leds.len() as u8;
    // for each led, write the state
    for p in power..0 {
        let state = num / p;
        num = num % p;

        // rprintf("State for led: {p} is {state}");

        // set the led status
        let led = leds[p as usize].borrow_mut();
        // match (state) {
        //     0 => led.set_low(),
        //     1 => led.set_high(),
        //     n => led.set_high(),
        //     // default => {
        //     //     led.set_high()
        //     //     // rprintln!("State is: {default}");
        //     // }
        // };

        if state == 0 {
            led.set_low().unwrap();
        } else {
            led.set_high().unwrap();
        }
    }
}
#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (Peripherals::take(), c_m_Peripherals::take()) {
        // surround the block in an interrupt free env
        cortex_m::interrupt::free(move |cs| {
            // // Enable clock for SYSCFG
            // let rcc = dp.RCC;
            // rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());
            let mut flash = dp.FLASH;
            let mut rcc = dp.RCC.configure().sysclk(8.mhz()).freeze(&mut flash);

            // setup the gpio
            let gpioa = dp.GPIOA.split(&mut rcc);
            let gpiob = dp.GPIOB.split(&mut rcc);

            // Initialise delay provider
            // let delay = Delay::new(cp.SYST, &rcc);
            /*
             * Connections are:
             *
             *  PB0 - PB7 are the LEDs that will be used to display the binary counter for the number of steps
             *  PA0 is the button to switch modes (speed/position)
             *  PA5 is a potentiometer to control the speed/position of the stepper motor
             */
            // setup button on PA0
            let usr_btn = gpioa.pa0.into_pull_down_input(cs);
            let mut pot = gpioa.pa5.into_analog(cs);

            // setup a binary counter that we will display on the LED's connected to PB0PB7
            // let mut step_counter = 0;
            let (led0, led1, led2, led3, led4, led5, led6, led7) = (
                gpiob.pb0.into_push_pull_output(cs),
                gpiob.pb1.into_push_pull_output(cs),
                gpiob.pb2.into_push_pull_output(cs),
                gpiob.pb3.into_push_pull_output(cs),
                gpiob.pb4.into_push_pull_output(cs),
                gpiob.pb5.into_push_pull_output(cs),
                gpiob.pb6.into_push_pull_output(cs),
                gpiob.pb7.into_push_pull_output(cs),
            );

            let mut led_array = [
                led0.downgrade(),
                led1.downgrade(),
                led2.downgrade(),
                led3.downgrade(),
                led4.downgrade(),
                led5.downgrade(),
                led6.downgrade(),
                led7.downgrade(),
            ];
            let num_leds = led_array.len();
            let max_displayable_num = (2 ^ num_leds as i32) - 1;

            test_leds(&mut led_array);
            let mut delay = Delay::new(cp.SYST, &rcc);
            // Initialise ADC
            let mut adc = hal::adc::Adc::new(dp.ADC, &mut rcc);

            loop {
                /*
                 * check if button is pressed
                 * if yes
                 *      increment counter
                 *      display on leds
                 *      sleep for value of potentiometer
                 */
                let btn_state = match usr_btn.is_high() {
                    Ok(btn) => {
                        hprintln!("Button state is: {btn}").unwrap();
                        btn
                    }
                    Err(err) => panic!("Cannot retrieve the state of the button, reason:\n{}", err),
                };

                let pot_val: u16 = adc.read(&mut pot).unwrap();

                if btn_state {
                    // if step_counter >= max_displayable_num {
                    //     hprintln!(
                    //         "Step counter is {} which overflows {}",
                    //         step_counter,
                    //         max_displayable_num
                    //     )
                    //     .unwrap();

                    //     step_counter = 0;
                    // } else {
                    //     step_counter = step_counter + 1;
                    // }

                    // TODO: overflow the step counter if it's too large
                    // display counter on leds
                    // write_to_leds(step_counter, &mut led_array);

                    //scale down value from adc into a u8
                    let led_val = (pot_val * 2 ^ 8 / 2 ^ 16) as u8;
                    hprintln!("Scaling down pot val from {pot_val} to {led_val}").unwrap();
                    write_to_leds(led_val, &mut led_array);

                    // 8MHz crystal --> sleep for 1 second is 8_000_000
                    // sleep
                    hprintln!("Sleeping for 0.5s").unwrap();

                    delay.delay_ms(500_u16);
                }
            }
        });
    };

    loop {
        continue;
    }
}
