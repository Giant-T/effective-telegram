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
    // Constructeur pour le buzzer passif.
    pub fn new(pin: Pin<mode::PwmOutput<TC>, PIN>) -> Self {
        return Self {
            pin,
            is_enabled: false,
        };
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    // Joue une note
    pub fn play(&mut self) {
        self.pin.set_duty(100);
        self.pin.enable();
        self.is_enabled = true;
    }

    // Arrete tout les sons sortants de cette pin
    pub fn stop(&mut self) {
        self.pin.disable();
        self.is_enabled = false;
    }
}
