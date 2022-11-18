use arduino_hal::{
    port::{mode, Pin},
    simple_pwm::PwmPinOps,
};

pub struct ColoredLed<TC1, PIN1, TC2, PIN2, TC3, PIN3> {
    red_pin: Pin<mode::PwmOutput<TC1>, PIN1>,
    green_pin: Pin<mode::PwmOutput<TC2>, PIN2>,
    blue_pin: Pin<mode::PwmOutput<TC3>, PIN3>,
}

impl<TC1, PIN1: PwmPinOps<TC1>, TC2, PIN2: PwmPinOps<TC2>, TC3, PIN3: PwmPinOps<TC3>>
    ColoredLed<TC1, PIN1, TC2, PIN2, TC3, PIN3>
{
    pub fn new(
        red_pin: Pin<mode::PwmOutput<TC1>, PIN1>,
        green_pin: Pin<mode::PwmOutput<TC2>, PIN2>,
        blue_pin: Pin<mode::PwmOutput<TC3>, PIN3>,
    ) -> Self {
        return Self {
            red_pin,
            green_pin,
            blue_pin,
        };
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
