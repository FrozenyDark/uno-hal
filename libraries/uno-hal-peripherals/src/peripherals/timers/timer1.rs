use crate::timers::registers::{
    ocr::{Ocr1A, Ocr1B},
    tccr::{Tccr1A, Tccr1B},
    tcnt::Tcnt1,
    tifr::Tifr1,
    timsk::Timsk1,
};

pub struct Timer1 {
    pub tccr1a: Tccr1A,
    pub tccr1b: Tccr1B,
    pub ocr1a: Ocr1A,
    pub ocr1b: Ocr1B,
    pub tcnt: Tcnt1,
    pub tifr: Tifr1,
    pub timsk: Timsk1,
}

#[repr(u8)]
pub enum ClockSelect1 {
    Stop = 0,
    Prescaler0 = 1,
    Prescaler8 = 2,
    Prescaler64 = 3, // Default
    Prescaler256 = 4,
    Prescaler1024 = 5,
    ExternalFalling = 6,
    ExternalRaising = 7,
}

#[repr(u8)]
pub enum WGMode1 {
    Normal = 0,
    PhaseCorrectPWM8 = 1, // Default
    PhaseCorrectPWM9 = 2,
    PhaseCorrectPWM10 = 3,
    ClearTimerOnCompare = 4,
    FastPWM8 = 5,
    FastPWM9 = 6,
    FastPWM10 = 7,
}

impl Timer1 {
    pub(crate) const fn new() -> Self {
        Self {
            tccr1a: Tccr1A::new(),
            tccr1b: Tccr1B::new(),
            ocr1a: Ocr1A::new(),
            ocr1b: Ocr1B::new(),
            tcnt: Tcnt1::new(),
            tifr: Tifr1::new(),
            timsk: Timsk1::new(),
        }
    }

    #[inline]
    pub unsafe fn take() -> Self {
        Self::new()
    }

    #[inline]
    pub fn setup_wgm(&mut self, setting: WGMode1) {
        let mask = setting as u8;

        unsafe {
            self.tccr1a.wgm10.set_mask(mask);
            self.tccr1a.wgm11.set_mask(mask);
            self.tccr1b.wgm12.set_mask(mask << 1);
        }
    }

    #[inline]
    pub fn setup_clock(&mut self, setting: ClockSelect1) {
        let mask = setting as u8;

        unsafe {
            self.tccr1b.cs10.set_mask(mask);
            self.tccr1b.cs11.set_mask(mask);
            self.tccr1b.cs12.set_mask(mask);
        }
    }

    #[inline]
    pub fn default_setup(&mut self) {
        self.setup_wgm(WGMode1::PhaseCorrectPWM8);
        self.setup_clock(ClockSelect1::Prescaler64);
    }
}
