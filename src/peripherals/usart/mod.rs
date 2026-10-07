mod interrupt;
pub mod readable;
mod worker;
pub mod writable;

use crate::peripherals::usart::{
    readable::Readable,
    worker::{tx_handler, RX_BUFFER, TX_BUFFER},
    writable::Writable,
};
use uno_hal_peripherals::{
    status::{atomic_block, Status},
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
}

impl Writable for HwSerial {
    fn write_c(&mut self, c: u8) -> usize {
        if c == 0 {
            return 0;
        }

        self.written = true;

        atomic_block(|cs| {
            let mut buffer = TX_BUFFER.borrow_ref_mut(cs);

            if buffer.is_empty() && self.usart.is_buffer_empty() {
                self.usart.write_byte(c);

                return 1;
            }

            if buffer.is_full() {
                while !self.usart.is_buffer_empty() {}

                tx_handler(&mut self.usart, &mut buffer);
            }

            buffer.add(c);
            self.usart.set_tx_interrupt(true);

            1
        })
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
                atomic_block(|cs| {
                    let mut buffer = TX_BUFFER.borrow_ref_mut(cs);
                    tx_handler(&mut self.usart, &mut buffer);
                });
            }
        }
    }

    #[inline]
    fn available_for_write(&self) -> usize {
        atomic_block(|cs| TX_BUFFER.borrow_ref(cs).count_empty()) as usize
    }
}

impl Readable for HwSerial {
    #[inline]
    fn peek_c(&self) -> Option<u8> {
        atomic_block(|cs| RX_BUFFER.borrow_ref(cs).peek())
    }

    #[inline]
    fn read_c(&self) -> Option<u8> {
        atomic_block(|cs| RX_BUFFER.borrow_ref_mut(cs).pop())
    }

    #[inline]
    fn available(&self) -> usize {
        atomic_block(|cs| RX_BUFFER.borrow_ref(cs).count_filled()) as usize
    }
}
