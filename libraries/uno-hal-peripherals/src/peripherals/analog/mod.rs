pub mod registers;

use registers::*;

use crate::register::{RO, RW};

pub const ANALOG_REFERENCE: u8 = 1; // DEFAULT

pub struct Analog {
    pub adc: Adc,
    pub adcsra: Adcsra,
    pub admux: Admux,
}

#[repr(u8)]
pub enum AnalogPrescaler {
    DivFactor2 = 1,
    DivFactor4 = 2,
    DivFactor8 = 3,
    DivFactor16 = 4,
    DivFactor32 = 5,
    DivFactor64 = 6,
    DivFactor128 = 7, // Default
}

impl Analog {
    pub(crate) const fn new() -> Self {
        Self {
            adc: Adc::new(),
            adcsra: Adcsra::new(),
            admux: Admux::new(),
        }
    }

    #[inline]
    pub fn setup_prescaler(&mut self, setting: AnalogPrescaler) {
        unsafe { self.adcsra.set(setting as u8) };
    }

    #[inline]
    pub fn enable_adc(&mut self) {
        unsafe { self.adcsra.set_bit(AdcsraBits::ADEN) };
    }

    #[inline]
    pub fn select_pin(&mut self, pin: u8) {
        unsafe {
            self.admux
                .reg_mut()
                .write((ANALOG_REFERENCE << 6) | (pin & 0x07))
        };
    }

    #[inline]
    pub fn read(&mut self) {
        unsafe { self.adcsra.set_bit(AdcsraBits::ADSC) };
        while self.adcsra.is_set_bit(AdcsraBits::ADSC) {}
    }

    #[inline]
    pub fn get_result(&self) -> u16 {
        self.adc.reg().read()
    }
}
