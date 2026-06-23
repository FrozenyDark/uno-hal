#![no_std]
#![no_main]

use uno_hal::{delay_ms, make_pins, Peripherals};

#[uno_hal::entry]
fn main() -> ! {
    let p = Peripherals::take_init().unwrap();
    let pins = make_pins!(p);

    let mut pin = pins.d13.into_output();
    let mut state = true;

    loop {
        pin.set(state);
        state = !state;
        delay_ms(1000);
    }
}
