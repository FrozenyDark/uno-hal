mod interrupt;
pub mod readable;
mod worker;
pub mod writable;

use crate::peripherals::usart::{readable::Readable, worker::USART_BUFFER, writable::Writable};
use uno_hal_peripherals::{
    atomic_block,
    status::Status,
    usart::{USARTSettings, Usart0},
};

pub struct HwSerial {
    written: bool,
    usart: Usart0,
}

impl HwSerial {
    #[inline]
    pub fn new(mut usart: Usart0, settings: USARTSettings) -> Self {
        usart.set_baud(settings);
        usart.set_format();
        usart.set_receive(true);
        usart.set_transmit(true);
        usart.set_rx_interrupt(true);
        usart.set_tx_interrupt(false);

        Self {
            written: false,
            usart,
        }
    }

    #[inline]
    pub fn free(mut self) -> Usart0 {
        self.flush();

        self.usart.set_receive(false);
        self.usart.set_transmit(false);
        self.usart.set_rx_interrupt(false);
        self.usart.set_tx_interrupt(false);

        self.usart
    }

    fn write(&mut self, byte: u8) -> usize {
        if byte == 0 {
            return 0;
        }

        self.written = true;

        if unsafe { USART_BUFFER.is_empty_tx() } && self.usart.is_buffer_empty() {
            atomic_block! {
                self.usart.write_byte(byte);
            }

            return 1;
        }

        while unsafe { USART_BUFFER.is_full_tx() } {
            if !Status::interrupts() && self.usart.is_buffer_empty() {
                unsafe { USART_BUFFER.tx_handler(&mut self.usart) };
            }
        }

        atomic_block! {
            unsafe { USART_BUFFER.add_tx(byte)};
            self.usart.set_tx_interrupt(true);
        }

        1
    }

    fn flush(&mut self) {
        if !self.written {
            return;
        }

        while self.usart.is_tx_interrupt_enabled() || !self.usart.is_tx_completed() {
            if !Status::interrupts()
                && self.usart.is_tx_interrupt_enabled()
                && self.usart.is_buffer_empty()
            {
                unsafe { USART_BUFFER.tx_handler(&mut self.usart) }
            }
        }
    }

    #[inline]
    fn available_for_write(&self) -> u8 {
        atomic_block! {
            unsafe { USART_BUFFER.available_tx() }
        }
    }

    #[inline]
    fn available(&self) -> u8 {
        atomic_block! {
            unsafe { USART_BUFFER.available_rx() }
        }
    }

    #[inline]
    fn peek(&self) -> Option<u8> {
        atomic_block! {
            unsafe { USART_BUFFER.peek_rx() }
        }
    }

    #[inline]
    fn read(&self) -> Option<u8> {
        atomic_block! {
            unsafe { USART_BUFFER.read_rx() }
        }
    }
}

impl Writable for HwSerial {
    #[inline]
    fn write_c(&mut self, c: u8) -> usize {
        self.write(c)
    }

    #[inline]
    fn flush(&mut self) {
        self.flush();
    }

    #[inline]
    fn available_for_write(&self) -> usize {
        self.available_for_write() as usize
    }
}

impl Readable for HwSerial {
    #[inline]
    fn peek_c(&self) -> Option<u8> {
        self.peek()
    }

    #[inline]
    fn read_c(&self) -> Option<u8> {
        self.read()
    }

    #[inline]
    fn available(&self) -> usize {
        self.available() as usize
    }
}
