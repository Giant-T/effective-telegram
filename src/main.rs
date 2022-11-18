#![no_std]
#![no_main]

use effective_telegram::led;

use arduino_hal::simple_pwm::IntoPwmPin;

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let pwm_timer3 = arduino_hal::simple_pwm::Timer3Pwm::new(
        dp.TC3,
        arduino_hal::simple_pwm::Prescaler::Prescale64,
    );
    let pwm_timer0 = arduino_hal::simple_pwm::Timer0Pwm::new(
        dp.TC0,
        arduino_hal::simple_pwm::Prescaler::Prescale64,
    );

    let mut colored_led = led::ColoredLed::new(
        pins.d2.into_output().into_pwm(&pwm_timer3),
        pins.d3.into_output().into_pwm(&pwm_timer3),
        pins.d4.into_output().into_pwm(&pwm_timer0),
    );

    colored_led.set_color(120, 50, 0);

    loop {
        colored_led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
