use uno_hal_registers::registers::timers::*;

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
    pub unsafe fn turn_off_pwm(&self) {
        match self {
            Self::Timer1A => TCCR1A.com1a1.clear(),
            Self::Timer1B => TCCR1A.com1b1.clear(),
            Self::Timer0A => TCCR0A.com0a1.clear(),
            Self::Timer0B => TCCR0A.com0b1.clear(),
            Self::Timer2A => TCCR2A.com2a1.clear(),
            Self::Timer2B => TCCR2A.com2b1.clear(),
        }
    }

    #[inline]
    pub unsafe fn turn_on_pwm(&self) {
        match self {
            Self::Timer1A => TCCR1A.com1a1.set(),
            Self::Timer1B => TCCR1A.com1b1.set(),
            Self::Timer0A => TCCR0A.com0a1.set(),
            Self::Timer0B => TCCR0A.com0b1.set(),
            Self::Timer2A => TCCR2A.com2a1.set(),
            Self::Timer2B => TCCR2A.com2b1.set(),
        }
    }

    #[inline]
    pub unsafe fn turn_off_pwm_from_pin(pin: u8) {
        if let Some(timer) = Self::get_timer(pin) {
            timer.turn_off_pwm();
        }
    }

    #[inline]
    pub unsafe fn turn_on_pwm_from_pin(pin: u8) {
        if let Some(timer) = Self::get_timer(pin) {
            timer.turn_on_pwm();
        }
    }

    pub const unsafe fn get_timer(pin: u8) -> Option<Timer> {
        match pin {
            3 => Some(Self::Timer2B),
            5 => Some(Self::Timer0B),
            6 => Some(Self::Timer0A),
            9 => Some(Self::Timer1A),
            10 => Some(Self::Timer1B),
            11 => Some(Self::Timer2A),
            _ => None,
        }
    }
}
