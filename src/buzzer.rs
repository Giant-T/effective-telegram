use arduino_hal::{
    port::{
        mode::{self},
        Pin,
    },
    simple_pwm::PwmPinOps,
};

#[derive(Clone, Copy)]
pub enum Note {
    C1 = 33,
    D1 = 37,
    A3 = 220,
}

#[derive(Clone, Copy)]
pub struct NoteWithDuration {
    note: Note,
    duration: u16,
}

impl NoteWithDuration {
    pub fn new(note: Note, duration: u16) -> Self {
        return Self { note, duration };
    }
}

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

    pub fn play_note(&mut self, note: Note) {
        self.pin.set_duty(note as u8);
        self.pin.enable();
        self.is_enabled = true;
    }

    pub fn stop(&mut self) {
        self.pin.disable();
        self.is_enabled = false;
    }

    pub fn play_music<const N: usize>(&mut self, music: &[NoteWithDuration; N]) {
        // Utilisation d'un foreach pour boucler au travers des valeurs du tableau.
        // || {} est la syntaxe d'une closure ce qui est pratiquement comme une expression lambda
        music.iter().for_each(|note_with_duration| {
            self.play_note(note_with_duration.note);
            arduino_hal::delay_ms(note_with_duration.duration);
        });

        self.stop();
    }
}
