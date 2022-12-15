#![no_std]
#![no_main]

use effective_telegram::{buzzer, display, led};

use arduino_hal::{prelude::_void_ResultVoidExt, simple_pwm::IntoPwmPin};

use embedded_hal::serial::Read;

use panic_halt as _;

enum Mode {
    Command,
    Password,
    Change,
}

#[arduino_hal::entry]
fn start() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    // Obtention de toutes les pins
    let pins = arduino_hal::pins!(dp);

    // Pour la communication avec le port serie
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

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
    colored_led.set_color(0, 0, 0xff);
    colored_led.enable();

    // Initialisation du passive buzzer
    let mut buzzer = buzzer::Passive::new(pins.d13.into_output().into_pwm(&pwm_timer0));
    buzzer.stop();

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

    let mut code: [u8; 4] = [1, 2, 3, 4];
    let mut code_index: usize = 0;
    let mut code_input: [u8; 4] = [0, 0, 0, 0];
    let mut mode: Mode = Mode::Command;

    loop {
        buzzer.stop();
        let input = nb::block!(serial.read()).void_unwrap() as u8;

        match mode {
            Mode::Command => {
                if input < 10 {
                    display.display(input);
                } else if input == 11 {
                    colored_led.set_color(255, 100, 0);
                    code_index = 0;
                    mode = Mode::Password;
                } else if input == 12 {
                    colored_led.set_color(120, 0, 120);
                    code_index = 0;
                    mode = Mode::Change;
                }
            }
            Mode::Password => {
                if input < 10 {
                    display.display(input);
                    code_input[code_index] = input;
                    code_index += 1;
                } else if input == 13 {
                    mode = Mode::Command;
                    colored_led.set_color(0, 0, 0xff);
                    ufmt::uwrite!(&mut serial, "{}", 1).void_unwrap();
                }
                if code_index > 3 {
                    mode = Mode::Command;
                    if code == code_input {
                        colored_led.set_color(0, 0xff, 0);
                    } else {
                        colored_led.set_color(0xff, 0, 0);
                        buzzer.play();
                    }
                    arduino_hal::delay_ms(1000);
                    colored_led.set_color(0, 0, 0xff);
                }
            }
            Mode::Change => {
                if input < 10 {
                    display.display(input);
                    code_input[code_index] = input;
                    code_index += 1;
                } else if input == 13 {
                    mode = Mode::Command;
                    colored_led.set_color(0, 0, 0xff);
                    ufmt::uwrite!(&mut serial, "{}", 1).void_unwrap();
                }
                if code_index > 3 {
                    mode = Mode::Command;
                    colored_led.set_color(0, 0xff, 0);
                    code = code_input;
                    arduino_hal::delay_ms(1000);
                    colored_led.set_color(0, 0, 0xff);
                }
            }
        }
    }
}
