use arduino_hal::{
    port::{
        mode::{self},
        Pin,
    },
    simple_pwm::PwmPinOps,
};

pub struct Passive<TC, PIN> {
    pin: Pin<mode::PwmOutput<TC>, PIN>,
    is_enabled: bool,
}

impl<TC, PIN: PwmPinOps<TC>> Passive<TC, PIN> {
    pub fn new(pin: Pin<mode::PwmOutput<TC>, PIN>) -> Self {
        return Self {
            pin,
            is_enabled: false,
        };
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn play(&mut self) {
        self.pin.set_duty(100);
        self.pin.enable();
        self.is_enabled = true;
    }

    pub fn stop(&mut self) {
        self.pin.disable();
        self.is_enabled = false;
    }
}
