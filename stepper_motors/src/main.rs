#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use cortex_m_rt::entry;

use stm32f0xx_hal as hal;

use hal::{pac, prelude::*, pwm};

fn flash_led(gpio: &mut Gpio<'info>, sleep: u32) {
    gpio.set_output();
    gpio.write(true);
    sleep.ms();
    gpio.write(false);
}

fn test_leds(gpio: &mut Gpio<'info>) {
    // test all the leds on the board
    for i in 0..8 {
        let pin = gpio. // TODO select each pb* pin for i
        flash_led(&pin, 200);
    }
}

#[entry]
fn main() -> ! {
    if let Some(mut dp) = pac::Peripherals::take() {
        // Set up the system clock.
        let mut rcc = dp.RCC.configure().sysclk(8.mhz()).freeze(&mut dp.FLASH);

        let gpioa = dp.GPIOA.split(&mut rcc);
        let gpiob = dp.GPIOB.split(&mut rcc);

        // setup button on PA0
        gpioa.pa0.into_input();

        // setup a binary counter that we will display on the LED's connected to PB0PB7
        let mut step_counter = 0;

        // TODO check these LEDs don't take ownership

        let mut led0 = gpiob.pb0.into_output();
        let mut led1 = gpiob.pb1.into_output();
        let mut led2 = gpiob.pb2.into_output();
        let mut led3 = gpiob.pb3.into_output();
        let mut led4 = gpiob.pb4.into_output();
        let mut led5 = gpiob.pb5.into_output();
        let mut led6 = gpiob.pb6.into_output();
        let mut led7 = gpiob.pb7.into_output();

        test_leds(&gpiob);

        // setup a timer that will increment the step counter every 100ms
        let mut step_counter_timer = tim1(dp.TIM1, &mut rcc, 100u32.ms());

        // TODO check if the below is neccessary
        step_counter_timer.enable();
        step_counter_timer.set_prescaler(1000000);
        step_counter_timer.set_counter(0);
        step_counter_timer.enable_irq();

        // let channels = cortex_m::interrupt::free(move |cs| {
        //     (
        //         gpioa.pa8.into_alternate_af2(cs),
        //         gpioa.pa9.into_alternate_af2(cs),
        //     )
        // });

        // let pwm = pwm::tim1(dp.TIM1, channels, &mut rcc, 20u32.khz());
        // let (mut ch1, _ch2) = pwm;
        // let max_duty = ch1.get_max_duty();
        // ch1.set_duty(max_duty / 2);
        // ch1.enable();
    }

    loop {
        // cortex_m::asm::nop();
    }
}
