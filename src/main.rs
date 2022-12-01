#![no_std]
#![no_main]

use effective_telegram::{
    buzzer::{self, Note, NoteWithDuration},
    display, led,
};

use arduino_hal::simple_pwm::IntoPwmPin;

use panic_halt as _;

#[arduino_hal::entry]
fn start() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Pour la communication avec le port serie
    // let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Utilise des timers puisque les pins pwm envoie du courant sur une intervale de temps
    let pwm_timer3 = arduino_hal::simple_pwm::Timer3Pwm::new(
        dp.TC3,
        arduino_hal::simple_pwm::Prescaler::Prescale64,
    );
    let pwm_timer0 = arduino_hal::simple_pwm::Timer0Pwm::new(
        dp.TC0,
        arduino_hal::simple_pwm::Prescaler::Prescale64,
    );

    // Initialisation de la led RGB
    let mut colored_led = led::ColoredLed::new(
        pins.d2.into_output().into_pwm(&pwm_timer3),
        pins.d3.into_output().into_pwm(&pwm_timer3),
        pins.d4.into_output().into_pwm(&pwm_timer0),
    );
    colored_led.set_color(0xff, 0, 0);

    // Initialisation du passive buzzer
    let mut buzzer = buzzer::Passive::new(pins.d13.into_output().into_pwm(&pwm_timer0));
    let music: [NoteWithDuration; 2] = [
        NoteWithDuration::new(Note::C1, 1000),
        NoteWithDuration::new(Note::A3, 2000),
    ];

    buzzer.play_music(&music);

    // Initialisation de l'afficheur a sept segment
    let mut display = display::SevenSegmentDisplay::new(
        pins.d10.into_output(),
        pins.d9.into_output(),
        pins.d7.into_output(),
        pins.d6.into_output(),
        pins.d5.into_output(),
        pins.d11.into_output(),
        pins.d12.into_output(),
        pins.d8.into_output(),
    );
    let mut number: u8 = 9;

    loop {
        colored_led.toggle();
        display.display(number);

        arduino_hal::delay_ms(1000);

        number -= 1;
        if number <= 0 {
            number = 9;
        }
    }
}
