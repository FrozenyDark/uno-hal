use crate::timers::registers::{
    ocr::{Ocr0A, Ocr0B},
    tccr::{Tccr0A, Tccr0B},
    tcnt::Tcnt0,
    tifr::Tifr0,
    timsk::Timsk0,
};

pub struct Timer0 {
    pub tccr0a: Tccr0A,
    pub tccr0b: Tccr0B,
    pub ocr0a: Ocr0A,
    pub ocr0b: Ocr0B,
    pub tcnt0: Tcnt0,
    pub tifr0: Tifr0,
    pub timsk0: Timsk0,
}

#[repr(u8)]
pub enum ClockSelect0 {
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
pub enum WGMode0 {
    Normal = 0,
    PhaseCorrectPWM = 1,
    ClearTimerOnCompare = 2,
    FastPWM = 3, // Default
}

impl Timer0 {
    pub(crate) const fn new() -> Self {
        Self {
            tccr0a: Tccr0A::new(),
            tccr0b: Tccr0B::new(),
            ocr0a: Ocr0A::new(),
            ocr0b: Ocr0B::new(),
            tcnt0: Tcnt0::new(),
            tifr0: Tifr0::new(),
            timsk0: Timsk0::new(),
        }
    }

    #[inline]
    pub unsafe fn take() -> Self {
        Self::new()
    }

    #[inline]
    pub fn setup_wgm(&mut self, setting: WGMode0) {
        let mask = setting as u8;

        unsafe {
            self.tccr0a.wgm00.set_mask(mask);
            self.tccr0a.wgm01.set_mask(mask);
        }
    }

    #[inline]
    pub fn setup_clock(&mut self, setting: ClockSelect0) {
        let mask = setting as u8;

        unsafe {
            self.tccr0b.cs00.set_mask(mask);
            self.tccr0b.cs01.set_mask(mask);
            self.tccr0b.cs02.set_mask(mask);
        }
    }

    #[inline]
    pub fn enable_overflow_interrupt(&mut self) {
        unsafe { self.timsk0.toie0.set() };
    }

    #[inline]
    pub fn default_setup(&mut self) {
        self.setup_wgm(WGMode0::FastPWM);
        self.setup_clock(ClockSelect0::Prescaler64);
        self.enable_overflow_interrupt();
    }
}
