#![no_std]
#![no_main]

use arduino_hal::{port::{Pin, mode}, simple_pwm::{IntoPwmPin, PwmPinOps}};
use panic_halt as _;

struct ColoredLed<TC1, PIN1, TC2, PIN2, TC3, PIN3> {
    red_pin: Pin<mode::PwmOutput<TC1>, PIN1>,
    green_pin: Pin<mode::PwmOutput<TC2>, PIN2>,
    blue_pin: Pin<mode::PwmOutput<TC3>, PIN3>,
}

impl<TC1, PIN1: PwmPinOps<TC1>, TC2, PIN2: PwmPinOps<TC2>, TC3, PIN3: PwmPinOps<TC3>> ColoredLed<TC1, PIN1, TC2, PIN2, TC3, PIN3> {
    pub fn new(red_pin: Pin<mode::PwmOutput<TC1>, PIN1>, green_pin: Pin<mode::PwmOutput<TC2>, PIN2>, blue_pin: Pin<mode::PwmOutput<TC3>, PIN3>) -> Self {
        return Self { red_pin, green_pin, blue_pin };
    }

    pub fn enable(&mut self) {
        self.red_pin.enable();
        self.green_pin.enable();
        self.blue_pin.enable();
    }

    pub fn disable(&mut self) {
        self.red_pin.disable();
        self.green_pin.disable();
        self.blue_pin.disable();
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.red_pin.set_duty(r);
        self.green_pin.set_duty(g);
        self.blue_pin.set_duty(b);
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let pwm_timer3 = 
        arduino_hal::simple_pwm::Timer3Pwm::new(dp.TC3, arduino_hal::simple_pwm::Prescaler::Prescale64);
    let pwm_timer0 = 
        arduino_hal::simple_pwm::Timer0Pwm::new(dp.TC0, arduino_hal::simple_pwm::Prescaler::Prescale64);

    let mut colored_led = ColoredLed::new(
        pins.d2.into_output().into_pwm(&pwm_timer3),
        pins.d3.into_output().into_pwm(&pwm_timer3),
        pins.d4.into_output().into_pwm(&pwm_timer0)
    );

    loop {
        colored_led.enable();
        colored_led.set_color(0, 255, 0);
        arduino_hal::delay_ms(1000);
        colored_led.disable();
        arduino_hal::delay_ms(1000);
    }
}
