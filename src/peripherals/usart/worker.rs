use uno_hal_peripherals::usart::Usart0;

use crate::volatile_cell::VolatileCell;

const BUFFER_SIZE: u8 = 16;

pub(super) static mut USART_BUFFER: UsartBuffer = UsartBuffer::new();

macro_rules! next {
    ($index:expr) => {
        ($index + 1) % BUFFER_SIZE
    };
}

pub(super) struct UsartBuffer {
    rx_head: VolatileCell<u8>,
    rx_tail: VolatileCell<u8>,
    tx_head: VolatileCell<u8>,
    tx_tail: VolatileCell<u8>,

    rx: [u8; BUFFER_SIZE as usize],
    tx: [u8; BUFFER_SIZE as usize],
}

impl UsartBuffer {
    pub const fn new() -> Self {
        Self {
            rx_head: VolatileCell::new(0),
            rx_tail: VolatileCell::new(0),
            tx_head: VolatileCell::new(0),
            tx_tail: VolatileCell::new(0),
            rx: [0; BUFFER_SIZE as usize],
            tx: [0; BUFFER_SIZE as usize],
        }
    }

    #[inline]
    pub(super) fn is_empty_tx(&self) -> bool {
        self.tx_head.read() == self.tx_tail.read()
    }

    #[inline]
    pub(super) fn is_full_tx(&self) -> bool {
        next!(self.tx_head.read()) == self.tx_tail.read()
    }

    #[inline]
    pub(super) fn add_tx(&mut self, byte: u8) {
        let head = self.tx_head.read();

        self.tx[head as usize] = byte;
        self.tx_head.write(next!(head));
    }

    #[inline]
    pub(super) fn available_tx(&self) -> u8 {
        let (head, tail) = (self.tx_head.read(), self.tx_tail.read());

        if head >= tail {
            BUFFER_SIZE - 1 - head + tail
        } else {
            tail - head - 1
        }
    }

    #[inline]
    pub(super) fn available_rx(&self) -> u8 {
        let (head, tail) = (self.rx_head.read(), self.rx_tail.read());

        (BUFFER_SIZE + head - tail) % BUFFER_SIZE
    }

    #[inline]
    pub(super) fn peek_rx(&self) -> Option<u8> {
        let (head, tail) = (self.rx_head.read(), self.rx_tail.read());

        if head == tail {
            None
        } else {
            Some(self.rx[tail as usize])
        }
    }

    #[inline]
    pub(super) fn read_rx(&mut self) -> Option<u8> {
        let (head, tail) = (self.rx_head.read(), self.rx_tail.read());

        if head == tail {
            None
        } else {
            let tail_next = next!(tail);
            self.rx_tail.write(tail_next);

            Some(self.rx[tail as usize])
        }
    }

    pub(super) fn tx_handler(&mut self, usart: &mut Usart0) {
        let (head, tail) = (self.tx_head.read(), self.tx_tail.read());

        let byte = self.tx[tail as usize];

        let tail = next!(tail);

        self.tx_tail.write(tail);

        usart.write_byte(byte);

        if head == tail {
            usart.set_tx_interrupt(false);
        }
    }

    pub(super) fn rx_handler(&mut self, usart: &Usart0) {
        if usart.parity_error() {
            let _ = usart.read_byte();
        }

        let (head, tail) = (self.rx_head.read(), self.rx_tail.read());
        let head_next = next!(head);

        let byte = usart.read_byte();

        if head_next != tail {
            self.rx[head as usize] = byte;
            self.rx_head.write(head_next);
        }
    }
}
