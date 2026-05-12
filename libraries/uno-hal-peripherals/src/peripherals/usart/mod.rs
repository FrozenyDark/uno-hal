pub mod registers;
mod usart_mode;

use crate::{
    addr::{RO, RW},
    usart::registers::{
        ubbr::Ubbr0,
        ucsr::{Ucsr0A, Ucsr0B, Ucsr0C},
        udr::Udr0,
    },
    F_CPU,
};
pub use usart_mode::*;

pub struct Usart0 {
    pub ubbr0: Ubbr0,
    pub ucsr0a: Ucsr0A,
    pub ucsr0b: Ucsr0B,
    pub ucsr0c: Ucsr0C,
    pub udr0: Udr0,
}

impl Usart0 {
    pub(crate) const fn new() -> Self {
        Self {
            ubbr0: Ubbr0::new(),
            ucsr0a: Ucsr0A::new(),
            ucsr0b: Ucsr0B::new(),
            ucsr0c: Ucsr0C::new(),
            udr0: Udr0::new(),
        }
    }

    pub fn set_baud(&mut self, baud: u32) {
        let mut baud_setting = ((F_CPU / 4 / baud - 1) / 2) as u16;

        if ((F_CPU == 16000000) && (baud == 57600)) || (baud_setting > 4095) {
            baud_setting = ((F_CPU / 8 / baud - 1) / 2) as u16;
        } else {
            unsafe { self.ucsr0a.u2x0.set() };
        }

        unsafe { self.ubbr0.reg_mut().write(baud_setting) };
    }

    #[inline]
    pub fn set_format(&mut self) {
        unsafe { self.ucsr0c.reg_mut().write(DEFAULT_MODE) };
    }

    #[inline]
    pub fn enable_receive(&mut self) {
        unsafe { self.ucsr0b.rxen0.set() };
    }

    #[inline]
    pub fn enable_transmit(&mut self) {
        unsafe { self.ucsr0b.txen0.set() };
    }

    #[inline]
    pub fn enable_rx_interrupt(&mut self) {
        unsafe { self.ucsr0b.rxcie0.set() };
    }

    #[inline]
    pub fn set_tx_interrupt(&mut self, state: bool) {
        match state {
            true => unsafe { self.ucsr0b.udrie0.set() },
            false => unsafe { self.ucsr0b.udrie0.clear() },
        }
    }

    #[inline]
    pub fn is_tx_interrupt_enabled(&self) -> bool {
        self.ucsr0b.udrie0.is_set()
    }

    #[inline]
    pub fn is_tx_completed(&self) -> bool {
        self.ucsr0a.txc0.is_set()
    }

    #[inline]
    pub fn is_buffer_empty(&self) -> bool {
        self.ucsr0a.udre0.is_set()
    }

    #[inline]
    pub fn write_bit(&mut self, bit: u8) {
        unsafe { self.udr0.reg_mut().write(bit) };
    }

    #[inline]
    pub fn read_bit(&self) -> u8 {
        self.udr0.reg().read()
    }

    #[inline]
    pub fn parity_error(&self) -> bool {
        self.ucsr0a.upe0.is_set()
    }
}
