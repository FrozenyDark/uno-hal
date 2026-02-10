#![no_std]
#![no_main]

use uno_hal::{
    pins::{digital_write, pin_mode, Mode},
    time::delay,
};

#[uno_hal::entry]
fn main() -> ! {
    uno_hal::init!();

    let pin: u8 = 13;
    pin_mode(pin, Mode::Output).unwrap();

    loop {
        digital_write(pin, true).unwrap();
        delay(200);
        digital_write(pin, false).unwrap();
        delay(200);
    }
}
