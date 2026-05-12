use crate::timers::registers::{
    ocr::{Ocr2A, Ocr2B},
    tccr::{Tccr2A, Tccr2B},
    tcnt::Tcnt2,
    tifr::Tifr2,
    timsk::Timsk2,
};

pub struct Timer2 {
    pub tccr2a: Tccr2A,
    pub tccr2b: Tccr2B,
    pub ocr2a: Ocr2A,
    pub ocr2b: Ocr2B,
    pub tcnt2: Tcnt2,
    pub tifr2: Tifr2,
    pub timsk2: Timsk2,
}

#[repr(u8)]
pub enum ClockSelect2 {
    Stop = 0,
    Prescaler0 = 1,
    Prescaler8 = 2,
    Prescaler32 = 3,
    Prescaler64 = 4, // Default
    Prescaler128 = 5,
    Prescaler256 = 6,
    Prescaler1024 = 7,
}

#[repr(u8)]
pub enum WGMode2 {
    Normal = 0,
    PhaseCorrectPWM = 1, // Default
    ClearTimerOnCompare = 2,
    FastPWM = 3,
}

impl Timer2 {
    pub(crate) const fn new() -> Self {
        Self {
            tccr2a: Tccr2A::new(),
            tccr2b: Tccr2B::new(),
            ocr2a: Ocr2A::new(),
            ocr2b: Ocr2B::new(),
            tcnt2: Tcnt2::new(),
            tifr2: Tifr2::new(),
            timsk2: Timsk2::new(),
        }
    }

    #[inline]
    pub unsafe fn take() -> Self {
        Self::new()
    }

    #[inline]
    pub fn setup_wgm(&mut self, setting: WGMode2) {
        let mask = setting as u8;

        unsafe {
            self.tccr2a.wgm20.set_mask(mask);
            self.tccr2a.wgm21.set_mask(mask);
        }
    }

    #[inline]
    pub fn setup_clock(&mut self, setting: ClockSelect2) {
        let mask = setting as u8;

        unsafe {
            self.tccr2b.cs20.set_mask(mask);
            self.tccr2b.cs21.set_mask(mask);
            self.tccr2b.cs22.set_mask(mask);
        }
    }

    #[inline]
    pub fn default_setup(&mut self) {
        self.setup_wgm(WGMode2::PhaseCorrectPWM);
        self.setup_clock(ClockSelect2::Prescaler64);
    }
}
