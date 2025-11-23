use crate::registers::timers::*;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Timer {
    Timer0A,
    Timer0B,
    Timer1A,
    Timer1B,
    Timer2A,
    Timer2B,
}

impl Timer {
    #[inline]
    pub fn turn_off_pwm(&self) {
        match self {
            Self::Timer1A => TCCR1A.com1a1.clear(),
            Self::Timer1B => TCCR1A.com1b1.clear(),
            Self::Timer0A => TCCR0A.com0a0.clear(),
            Self::Timer0B => TCCR0A.com0b1.clear(),
            Self::Timer2A => TCCR2A.com2a1.clear(),
            Self::Timer2B => TCCR2A.com2b1.clear(),
        }
    }

    #[inline]
    pub fn turn_off_pwm_from_pin(pin: u8) {
        match pin {
            3 => Self::Timer2B.turn_off_pwm(),
            5 => Self::Timer0B.turn_off_pwm(),
            6 => Self::Timer0A.turn_off_pwm(),
            9 => Self::Timer1A.turn_off_pwm(),
            10 => Self::Timer1B.turn_off_pwm(),
            11 => Self::Timer2A.turn_off_pwm(),
            _ => {}
        }
    }
}
