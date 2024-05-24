// #![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use cortex_m_rt::entry;

use stm32f0xx_hal as hal;

// use crate::hal::{
//     delay::Delay,
//     gpio::*,
//     pac::{interrupt, Interrupt, Peripherals, EXTI},
//     prelude::*,
// };
// use cortex_m::{interrupt::Mutex, peripheral::Peripherals as c_m_Peripherals};
use crate::hal::{
    delay::Delay,
    gpio::*,
    pac::{interrupt, Interrupt, Peripherals, EXTI},
    prelude::*,
};

use cortex_m::{interrupt::Mutex, peripheral::Peripherals as c_m_Peripherals};

use core::{cell::RefCell, ops::DerefMut};

// If we're using interrupts
// // Make our LED globally available
// static LEDs: Mutex<RefCell<Option<[hal::gpio::Pin<hal::gpio::Output<hal::gpio::PushPull>>]>>> =
//     Mutex::new(RefCell::new(None));

// // Make our delay provider globally available
// static DELAY: Mutex<RefCell<Option<Delay>>> = Mutex::new(RefCell::new(None));

// // Make external interrupt registers globally available
// static INT: Mutex<RefCell<Option<EXTI>>> = Mutex::new(RefCell::new(None));

fn flash_led(led: &mut hal::gpio::Pin<hal::gpio::Output<hal::gpio::PushPull>>, sleep_ms: u16) {
    // Running with interrupts
    // // Enter critical section
    // cortex_m::interrupt::free(|cs| {
    //     // Obtain Mutex protected resources
    //     if let &mut Some(ref mut delay) = DELAY.borrow(cs).borrow_mut().deref_mut() {
    //         // Turn on LED
    //         led.set_high().ok();
    //         // Wait a bit
    //         delay.delay_ms(sleep_ms);
    //         // Turn off LED
    //         led.set_low().ok();
    //     }
    // });

    // Running without interrupts:
    // Turn on LED
    led.set_high().ok();
    // Wait a bit
    // TODO: delay.delay_ms(sleep_ms);
    // Turn off LED
    led.set_low().ok();
}

fn test_leds(leds: &mut [hal::gpio::Pin<hal::gpio::Output<hal::gpio::PushPull>>]) {
    for led in leds {
        flash_led(led, 500);
        flash_led(led, 500);
    }
}

// fn setup leds(gpiob: Parts) {
// fn setup_leds(gpio: ) {
//     // Consume the gpio's and set the used pins to the appropriate values
//     // let usr_button =gpioa.
//     let (led0, led1, led2, led3, led4, led5, led6, led7) = cortex_m::interrupt::free(move |cs| {
//         (
//             gpiob.pb0.into_push_pull_output(cs),
//             gpiob.pb1.into_push_pull_output(cs),
//             gpiob.pb2.into_push_pull_output(cs),
//             gpiob.pb3.into_push_pull_output(cs),
//             gpiob.pb4.into_push_pull_output(cs),
//             gpiob.pb5.into_push_pull_output(cs),
//             gpiob.pb6.into_push_pull_output(cs),
//             gpiob.pb7.into_push_pull_output(cs),
//         )
//     });

//     let mut led_array = [
//         led0.downgrade(),
//         led1.downgrade(),
//         led2.downgrade(),
//         led3.downgrade(),
//         led4.downgrade(),
//         led5.downgrade(),
//         led6.downgrade(),
//         led7.downgrade(),
//     ];

//     led_array
// }
#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (Peripherals::take(), c_m_Peripherals::take()) {
        cortex_m::interrupt::free(move |cs| {
            // Enable clock for SYSCFG
            let rcc = dp.RCC;
            rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());

            let mut flash = dp.FLASH;
            let mut rcc = rcc.configure().sysclk(8.mhz()).freeze(&mut flash);

            // setup the gpio
            let gpioa = dp.GPIOA.split(&mut rcc);
            let gpiob = dp.GPIOB.split(&mut rcc);
            let syscfg = dp.SYSCFG;
            let exti = dp.EXTI;

            // Initialise delay provider
            let delay = Delay::new(cp.SYST, &rcc);
            /*
             * Connections are:
             *
             *  PB0 - PB7 are the LEDs that will be used to display the binary counter for the number of steps
             *  PA0 is the button to switch modes (speed/position)
             *  PA5 is a potentiometer to control the speed/position of the stepper motor
             */
            // setup button on PA0
            // let mut button = cortex_m::interrupt(|cs| gpioa.pa0.into_pull_up_input(cs));

            // setup a binary counter that we will display on the LED's connected to PB0PB7
            let mut step_counter = 0;
            let (led0, led1, led2, led3, led4, led5, led6, led7) =
                cortex_m::interrupt::free(move |cs| {
                    (
                        gpiob.pb0.into_push_pull_output(cs),
                        gpiob.pb1.into_push_pull_output(cs),
                        gpiob.pb2.into_push_pull_output(cs),
                        gpiob.pb3.into_push_pull_output(cs),
                        gpiob.pb4.into_push_pull_output(cs),
                        gpiob.pb5.into_push_pull_output(cs),
                        gpiob.pb6.into_push_pull_output(cs),
                        gpiob.pb7.into_push_pull_output(cs),
                    )
                });

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

            test_leds(&mut led_array);

            // Below is for if we're using interrupts

            // Enable external interrupt for PB1
            syscfg.exticr1.modify(|_, w| unsafe { w.exti1().bits(1) });

            // Set interrupt request mask for line 1
            exti.imr.modify(|_, w| w.mr1().set_bit());

            // Set interrupt rising trigger for line 1
            exti.rtsr.modify(|_, w| w.tr1().set_bit());

            // Move control over LED and DELAY and EXTI into global mutexes
            // *DELAY.borrow(cs).borrow_mut() = Some(delay);
            // *INT.borrow(cs).borrow_mut() = Some(exti);
            // *LEDs.borrow(cs).borrow_mut() = Some(led_array);

            // // Enable EXTI IRQ, set prio 1 and clear any pending IRQs
            // let mut nvic = cp.NVIC;
            // unsafe {
            //     nvic.set_priority(Interrupt::EXTI0_1, 1);
            //     cortex_m::peripheral::NVIC::unmask(Interrupt::EXTI0_1);
            // }
            // cortex_m::peripheral::NVIC::unpend(Interrupt::EXTI0_1);
        });
    };

    loop {
        continue;
    }
}

/**
 * Interrupt handler for the user button press
 */
fn button_handler() {
    todo!("Button handler");
    // read the button state
    // let button_state = gpioa.pa0.read();
    // println!("Button state: {:?}", button_state);
}
