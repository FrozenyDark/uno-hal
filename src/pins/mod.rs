mod common;
mod mode;
mod port;
mod timer;

use crate::{
    atomic_block,
    pins::{port::Port, timer::Timer},
    registers::{analog::*, Register, ANALOG_REFERENCE},
};
use common::*;

pub use mode::*;

pub fn pin_mode(pin: u8, mode: Mode) -> Result<(), &'static str> {
    let bit = pin_to_bit(pin)?;
    let port = Port::try_from(pin)?;

    let reg = port.mode_reg();
    let out = port.out_reg();

    atomic_block!(match mode {
        Mode::Input => {
            reg.clear(bit);
            out.clear(bit);
        }
        Mode::InputPullup => {
            reg.clear(bit);
            out.set(bit);
        }
        Mode::Output => reg.set(bit),
    });

    Ok(())
}

pub fn digital_write(pin: u8, val: bool) -> Result<(), &'static str> {
    let bit = pin_to_bit(pin)?;
    let port = Port::try_from(pin)?;

    Timer::turn_off_pwm_from_pin(pin);

    let out = port.out_reg();

    atomic_block! {
        match val {
            false => out.clear(bit),
            true => out.set(bit)
        }
    };

    Ok(())
}

pub fn digital_read(pin: u8) -> Result<bool, &'static str> {
    let bit = pin_to_bit(pin)?;
    let port = Port::try_from(pin)?;

    Timer::turn_off_pwm_from_pin(pin);

    Ok(port.in_reg().is_set(bit))
}

pub fn analog_read(mut pin: u8) -> Result<u16, &'static str> {
    if !(14..20).contains(&pin) {
        return Err("Invalid pin");
    }
    pin -= 14;

    ADMUX.write((ANALOG_REFERENCE << 6) | (pin & 0x07));

    ADCSRA.adsc.set();

    while ADCSRA.adsc.is_set() {}

    Ok(ADC.read())
}
