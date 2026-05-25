use uno_hal_peripherals::{atomic_block, status::Sreg, usart::Usart0};

use crate::volatile_cell::VolatileCell;

const SERIAL_RX_BUFFER_SIZE: u8 = 16;
const SERIAL_TX_BUFFER_SIZE: u8 = 16;

pub(super) static mut USART_WORKER: Option<UsartWorker> = None;

pub struct UsartWorker {
    written: bool,
    usart: Usart0,

    rx_head: VolatileCell<u8>,
    rx_tail: VolatileCell<u8>,
    tx_head: VolatileCell<u8>,
    tx_tail: VolatileCell<u8>,

    rx: [u8; SERIAL_RX_BUFFER_SIZE as usize],
    tx: [u8; SERIAL_TX_BUFFER_SIZE as usize],
}

impl UsartWorker {
    #[inline]
    pub(super) fn create(usart: Usart0) {
        let worker = Self {
            written: false,
            usart,
            rx_head: VolatileCell::new(0),
            rx_tail: VolatileCell::new(0),
            tx_head: VolatileCell::new(0),
            tx_tail: VolatileCell::new(0),
            rx: [0; SERIAL_RX_BUFFER_SIZE as usize],
            tx: [0; SERIAL_TX_BUFFER_SIZE as usize],
        };

        unsafe { USART_WORKER.replace(worker) };
    }

    #[inline]
    pub(super) fn begin(&mut self, baud: u32) {
        self.usart.set_baud(baud);
        self.usart.set_format();
        self.usart.enable_receive();
        self.usart.enable_transmit();
        self.usart.enable_rx_interrupt();
        self.usart.set_tx_interrupt(false);
    }

    pub(super) fn write(&mut self, bit: u8) -> usize {
        if bit == 0 {
            return 0;
        }

        self.written = true;

        if self.tx_head.read() == self.tx_tail.read() && self.usart.is_buffer_empty() {
            atomic_block! {
                self.usart.write_bit(bit);
            }

            return 1;
        }

        let i = (self.tx_head.read() + 1) % SERIAL_TX_BUFFER_SIZE;

        while i == self.tx_tail.read() {
            if Sreg.bits().sreg_i.is_clear() && self.usart.is_buffer_empty() {
                self.tx_interrupt();
            }
        }

        let head = self.tx_head.read() as usize;
        self.tx[head] = bit;

        atomic_block! {
            self.tx_head.write(i);
            self.usart.set_tx_interrupt(true);
        }

        1
    }

    pub(super) fn flush(&mut self) {
        if !self.written {
            return;
        }

        while self.usart.is_tx_interrupt_enabled() || !self.usart.is_tx_completed() {
            if Sreg.bits().sreg_i.is_clear()
                && self.usart.is_tx_interrupt_enabled()
                && self.usart.is_buffer_empty()
            {
                self.tx_interrupt();
            }
        }
    }

    pub(super) fn available_for_write(&self) -> u8 {
        let (head, tail) = atomic_block! {
            (self.tx_head.read(), self.tx_tail.read())
        };

        if head >= tail {
            SERIAL_TX_BUFFER_SIZE - 1 - head + tail
        } else {
            tail - head - 1
        }
    }

    pub(super) fn tx_interrupt(&mut self) {
        let tail = self.tx_tail.read() as usize;
        let bit = self.tx[tail];
        self.tx_tail.update(|x| (x + 1) % SERIAL_TX_BUFFER_SIZE);

        self.usart.write_bit(bit);

        if self.tx_head.read() == self.tx_tail.read() {
            self.usart.set_tx_interrupt(false);
        }
    }

    pub(super) fn available(&self) -> usize {
        let (head, tail) = atomic_block! {
            (self.rx_head.read() as usize, self.rx_tail.read() as usize)
        };
        const SIZE: usize = SERIAL_RX_BUFFER_SIZE as usize;

        (SIZE + head - tail) % SIZE
    }

    pub(super) fn peek(&self) -> Option<u8> {
        atomic_block! {
            if self.rx_head.read() == self.rx_tail.read() {
                None
            } else {
                Some(self.rx[self.rx_tail.read() as usize])
            }
        }
    }

    pub(super) fn read(&mut self) -> Option<u8> {
        if let Some(bit) = self.peek() {
            atomic_block! {
                self.rx_tail.update(|x| (x + 1) % SERIAL_RX_BUFFER_SIZE);
            }
            Some(bit)
        } else {
            None
        }
    }

    pub(super) fn rx_interrupt(&mut self) {
        if self.usart.parity_error() {
            let _ = self.usart.read_bit();
        }

        let bit = self.usart.read_bit();
        let i = (self.rx_head.read() + 1) % SERIAL_RX_BUFFER_SIZE;

        if i != self.rx_tail.read() {
            self.rx[self.rx_head.read() as usize] = bit;
            self.rx_head.write(i);
        }
    }
}
