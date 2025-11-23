use crate::{
    atomic_block,
    registers::{interrupt::SREG, usart::*, Register, F_CPU},
    volatile_cell::VolatileCell,
};

const SERIAL_8N1: u8 = 0x06;

const UBRRH: UBRR0H = UBRR0H;
const UBRRL: UBRR0L = UBRR0L;
const UCSRA: UCSR0A = UCSR0A;
const UCSRB: UCSR0B = UCSR0B;
const UCSRC: UCSR0C = UCSR0C;
const UDR: UDR0 = UDR0;

const U2X0: u8 = UCSRA.u2x0.bit();
const MPCM0: u8 = UCSRA.mpcm0.bit();
const TXC0: u8 = UCSRA.txc0.bit();

const SERIAL_TX_BUFFER_SIZE: u8 = 16;
const SERIAL_RX_BUFFER_SIZE: u8 = 16;

pub(super) static mut SERIAL_WORKER: HardwareSerialWorker = HardwareSerialWorker::new();

pub struct HardwareSerialWorker {
    written: bool,

    rx_buffer_head: VolatileCell<u8>,
    rx_buffer_tail: VolatileCell<u8>,
    rx_buffer: [u8; SERIAL_RX_BUFFER_SIZE as usize],

    tx_buffer_head: VolatileCell<u8>,
    tx_buffer_tail: VolatileCell<u8>,
    tx_buffer: [u8; SERIAL_TX_BUFFER_SIZE as usize],
}

impl HardwareSerialWorker {
    #[inline]
    const fn new() -> Self {
        Self {
            written: false,

            rx_buffer_head: VolatileCell::new(0),
            rx_buffer_tail: VolatileCell::new(0),
            rx_buffer: [0; SERIAL_RX_BUFFER_SIZE as usize],

            tx_buffer_head: VolatileCell::new(0),
            tx_buffer_tail: VolatileCell::new(0),
            tx_buffer: [0; SERIAL_TX_BUFFER_SIZE as usize],
        }
    }

    pub(super) fn begin(&self, baud: u32) {
        let mut baud_setting = ((F_CPU / 4 / baud - 1) / 2) as u16;
        UCSRA.write(1 << U2X0);

        if ((F_CPU == 16000000) && (baud == 57600)) || (baud_setting > 4095) {
            UCSRA.write(0);
            baud_setting = ((F_CPU / 8 / baud - 1) / 2) as u16;
        }

        UBRRH.write((baud_setting >> 8) as u8);
        UBRRL.write(baud_setting as u8);

        UCSRC.write(SERIAL_8N1);

        UCSRB.rxen0.set();
        UCSRB.txen0.set();
        UCSRB.rxcie0.set();
        UCSRB.udrie0.clear();
    }

    #[inline(never)]
    pub(super) fn write(&mut self, c: u8) -> usize {
        if c == 0 {
            return 0;
        }

        self.written = true;

        if self.tx_buffer_head.read() == self.tx_buffer_tail.read() && UCSRA.udre0.is_set() {
            atomic_block!({
                UDR.write(c);
                UCSRA.update(|x| (x & ((1 << U2X0) | (1 << MPCM0))) | (1 << TXC0));
            });
            return 1;
        }
        let i = (self.tx_buffer_head.read() + 1) % SERIAL_TX_BUFFER_SIZE;

        while i == self.tx_buffer_tail.read() {
            if SREG.sreg_i.is_clear() {
                if UCSRA.udre0.is_set() {
                    self.tx_udr_empty_irq();
                }
            } else {
                // nop
            }
        }

        let head = self.tx_buffer_head.read() as usize;
        self.tx_buffer[head] = c;

        atomic_block!({
            self.tx_buffer_head.write(i);
            UCSRB.udrie0.set();
        });

        1
    }

    #[inline(never)]
    pub(super) fn flush(&mut self) {
        if !self.written {
            return;
        }

        while UCSRB.udrie0.is_set() || UCSRA.txc0.is_clear() {
            if SREG.sreg_i.is_clear() && UCSRB.udrie0.is_set() && UCSRA.udre0.is_set() {
                self.tx_udr_empty_irq();
            }
        }
    }

    #[inline(never)]
    pub(super) fn available_for_write(&self) -> u8 {
        let head;
        let tail;

        atomic_block!({
            head = self.tx_buffer_head.read();
            tail = self.tx_buffer_tail.read();
        });

        if head >= tail {
            SERIAL_TX_BUFFER_SIZE - 1 - head + tail
        } else {
            tail - head - 1
        }
    }

    pub(super) fn tx_udr_empty_irq(&mut self) {
        let tail = self.tx_buffer_tail.read() as usize;
        let c = self.tx_buffer[tail];
        self.tx_buffer_tail
            .update(|x| (x + 1) % SERIAL_TX_BUFFER_SIZE);

        UDR.write(c);

        UCSRA.update(|x| (x & ((1 << U2X0) | (1 << MPCM0))) | (1 << TXC0));

        if self.tx_buffer_head.read() == self.tx_buffer_tail.read() {
            UCSRB.udrie0.clear();
        }
    }
}
