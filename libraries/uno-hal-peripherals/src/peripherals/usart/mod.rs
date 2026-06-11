pub mod registers;
mod usart_mode;

use crate::{
    register::{RO, RW},
    usart::registers::{
        ubbr::Ubbr0,
        ucsr::{Ucsr0A, Ucsr0ABits, Ucsr0B, Ucsr0BBits, Ucsr0C},
        udr::Udr0,
    },
    F_CPU,
};
pub use usart_mode::*;

pub struct USARTSettings {
    pub baud: u32,
    pub double_speed: bool,
}

impl Default for USARTSettings {
    fn default() -> Self {
        Self {
            baud: 57600,
            double_speed: false,
        }
    }
}

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

    pub fn set_baud(&mut self, settings: USARTSettings) {
        let mut divider = 8;

        if settings.double_speed {
            divider = 4;
            unsafe { self.ucsr0a.set_bit(Ucsr0ABits::U2X0) };
        }

        let baud_setting = ((F_CPU / divider / settings.baud - 1) / 2) as u16;

        unsafe { self.ubbr0.reg_mut().write(baud_setting) };
    }

    #[inline]
    pub fn set_format(&mut self) {
        unsafe { self.ucsr0c.reg_mut().write(DEFAULT_MODE) };
    }

    #[inline]
    pub fn set_receive(&mut self, state: bool) {
        match state {
            true => unsafe { self.ucsr0b.set_bit(Ucsr0BBits::RXEN0) },
            false => unsafe { self.ucsr0b.clear_bit(Ucsr0BBits::RXEN0) },
        }
    }

    #[inline]
    pub fn set_transmit(&mut self, state: bool) {
        match state {
            true => unsafe { self.ucsr0b.set_bit(Ucsr0BBits::TXEN0) },
            false => unsafe { self.ucsr0b.clear_bit(Ucsr0BBits::TXEN0) },
        }
    }

    #[inline]
    pub fn set_rx_interrupt(&mut self, state: bool) {
        match state {
            true => unsafe { self.ucsr0b.set_bit(Ucsr0BBits::RXCIE0) },
            false => unsafe { self.ucsr0b.clear_bit(Ucsr0BBits::RXCIE0) },
        }
    }

    #[inline]
    pub fn set_tx_interrupt(&mut self, state: bool) {
        match state {
            true => unsafe { self.ucsr0b.set_bit(Ucsr0BBits::UDRIE0) },
            false => unsafe { self.ucsr0b.clear_bit(Ucsr0BBits::UDRIE0) },
        }
    }

    #[inline]
    pub fn is_tx_interrupt_enabled(&self) -> bool {
        self.ucsr0b.is_set_bit(Ucsr0BBits::UDRIE0)
    }

    #[inline]
    pub fn is_tx_completed(&self) -> bool {
        self.ucsr0a.is_set_bit(Ucsr0ABits::TXC0)
    }

    #[inline]
    pub fn is_buffer_empty(&self) -> bool {
        self.ucsr0a.is_set_bit(Ucsr0ABits::UDRE0)
    }

    #[inline]
    pub fn write_byte(&mut self, byte: u8) {
        unsafe { self.udr0.reg_mut().write(byte) };
    }

    #[inline]
    pub fn read_byte(&self) -> u8 {
        self.udr0.reg().read()
    }

    #[inline]
    pub fn parity_error(&self) -> bool {
        self.ucsr0a.is_set_bit(Ucsr0ABits::UPE0)
    }
}
