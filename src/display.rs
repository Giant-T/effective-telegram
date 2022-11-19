use arduino_hal::{
    hal::port::Dynamic,
    port::{mode::Output, Pin, PinOps},
};

const NUMBERS: [[bool; 8]; 10] = [
    [false, true, true, true, true, true, true, false], // 0
    [false, false, true, true, false, false, false, false], // 1
    [false, true, true, false, true, true, false, true], // 2
    [false, true, true, true, true, false, false, true], // 3
    [false, false, true, true, false, false, true, true], // 4
    [false, true, false, true, true, false, true, true], // 5
    [false, true, false, true, true, true, true, true], // 6
    [false, true, true, true, false, false, false, false], // 7
    [false, true, true, true, true, true, true, true],  // 8
    [false, true, true, true, true, false, true, true], // 9
];

pub struct SevenSegmentDisplay<PINA, PINB, PINC, PIND, PINE, PINF, PING, PINDP> {
    pin_a: Pin<Output, PINA>,
    pin_b: Pin<Output, PINB>,
    pin_c: Pin<Output, PINC>,
    pin_d: Pin<Output, PIND>,
    pin_e: Pin<Output, PINE>,
    pin_f: Pin<Output, PINF>,
    pin_g: Pin<Output, PING>,
    pin_dp: Pin<Output, PINDP>,
}

impl<
        PINA: PinOps,
        PINB: PinOps,
        PINC: PinOps,
        PIND: PinOps,
        PINE: PinOps,
        PINF: PinOps,
        PING: PinOps,
        PINDP: PinOps,
    > SevenSegmentDisplay<PINA, PINB, PINC, PIND, PINE, PINF, PING, PINDP>
{
    pub fn new(
        pin_a: Pin<Output, PINA>,
        pin_b: Pin<Output, PINB>,
        pin_c: Pin<Output, PINC>,
        pin_d: Pin<Output, PIND>,
        pin_e: Pin<Output, PINE>,
        pin_f: Pin<Output, PINF>,
        pin_g: Pin<Output, PING>,
        pin_dp: Pin<Output, PINDP>,
    ) -> Self {
        return SevenSegmentDisplay {
            pin_a,
            pin_b,
            pin_c,
            pin_d,
            pin_e,
            pin_f,
            pin_g,
            pin_dp,
        };
    }

    pub fn display(&mut self, number: u8) {
        Self::change_segment(NUMBERS[number as usize][0], &mut self.pin_dp);
        Self::change_segment(NUMBERS[number as usize][1], &mut self.pin_a);
        Self::change_segment(NUMBERS[number as usize][2], &mut self.pin_b);
        Self::change_segment(NUMBERS[number as usize][3], &mut self.pin_c);
        Self::change_segment(NUMBERS[number as usize][4], &mut self.pin_d);
        Self::change_segment(NUMBERS[number as usize][5], &mut self.pin_e);
        Self::change_segment(NUMBERS[number as usize][6], &mut self.pin_f);
        Self::change_segment(NUMBERS[number as usize][7], &mut self.pin_g);
    }

    fn change_segment<PIN: PinOps>(should_be_high: bool, pin: &mut Pin<Output, PIN>) {
        if should_be_high {
            pin.set_high();
        } else {
            pin.set_low();
        }
    }
}
