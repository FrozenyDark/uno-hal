mod common;
mod mode;
mod port;
mod timer;

use crate::pins::{port::Port, timer::Timer};
use common::*;
use uno_hal_registers::{
    atomic_block,
    registers::{
        analog::*,
        timers::{OCR0A, OCR0B, OCR1A, OCR1B, OCR2A, OCR2B},
        Register, ANALOG_REFERENCE,
    },
};

pub use mode::*;

pub unsafe fn pin_mode(pin: u8, mode: Mode) -> Result<(), &'static str> {
    let bit = pin_to_bit(pin)?;
    let port = Port::try_from(pin)?;

    let reg = port.mode_reg();
    let out = port.out_reg();

    atomic_block! {
        match mode {
            Mode::Input => {
                reg.clear(bit);
                out.clear(bit);
            }
            Mode::InputPullup => {
                reg.clear(bit);
                out.set(bit);
            }
            Mode::Output => reg.set(bit),
        }
    };

    Ok(())
}

pub unsafe fn digital_write(pin: u8, val: bool) -> Result<(), &'static str> {
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

pub unsafe fn digital_read(pin: u8) -> Result<bool, &'static str> {
    let bit = pin_to_bit(pin)?;
    let port = Port::try_from(pin)?;

    Timer::turn_off_pwm_from_pin(pin);

    Ok(port.in_reg().is_set(bit))
}

pub unsafe fn analog_read(mut pin: u8) -> Result<u16, &'static str> {
    if !(14..20).contains(&pin) {
        return Err("Invalid pin");
    }
    pin -= 14;

    ADMUX.write((ANALOG_REFERENCE << 6) | (pin & 0x07));

    ADCSRA.adsc.set();

    while ADCSRA.adsc.is_set() {}

    Ok(ADC.read())
}

pub unsafe fn analog_write(pin: u8, val: u8) -> Result<(), &'static str> {
    pin_mode(pin, Mode::Output)?;

    if val == u8::MIN {
        digital_write(pin, false)?;
        return Ok(());
    } else if val == u8::MAX {
        digital_write(pin, true)?;
        return Ok(());
    }

    match Timer::get_timer(pin) {
        Some(Timer::Timer0A) => {
            Timer::Timer0A.turn_on_pwm();
            OCR0A.write(val);
        }
        Some(Timer::Timer0B) => {
            Timer::Timer0B.turn_on_pwm();
            OCR0B.write(val);
        }
        Some(Timer::Timer1A) => {
            Timer::Timer1A.turn_on_pwm();
            OCR1A.write(val as u16);
        }
        Some(Timer::Timer1B) => {
            Timer::Timer1B.turn_on_pwm();
            OCR1B.write(val as u16);
        }
        Some(Timer::Timer2A) => {
            Timer::Timer2A.turn_on_pwm();
            OCR2A.write(val);
        }
        Some(Timer::Timer2B) => {
            Timer::Timer2B.turn_on_pwm();
            OCR2B.write(val);
        }
        None => {
            if val < 128 {
                digital_write(pin, false)?;
            } else {
                digital_write(pin, true)?;
            }
        }
    }

    Ok(())
}
