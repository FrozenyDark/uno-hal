use crate::{
    atomic_block,
    hardware_serial::print::Print,
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

pub(super) static mut HARDWARE_SERIAL: HardwareSerialInternal =
    HardwareSerialInternal::static_new();

#[derive(Debug)]
pub(super) struct HardwareSerialInternal {
    written: bool,
    _rx_buffer_head: VolatileCell<u8>,
    _rx_buffer_tail: VolatileCell<u8>,
    _tx_buffer_head: VolatileCell<u8>,
    _tx_buffer_tail: VolatileCell<u8>,
    _tx_buffer: [u8; SERIAL_TX_BUFFER_SIZE as usize],
    _rx_buffer: [u8; SERIAL_RX_BUFFER_SIZE as usize],
}

impl HardwareSerialInternal {
    const fn static_new() -> Self {
        Self {
            written: false,
            _rx_buffer_head: VolatileCell::new(0),
            _rx_buffer_tail: VolatileCell::new(0),
            _tx_buffer_head: VolatileCell::new(0),
            _tx_buffer_tail: VolatileCell::new(0),
            _tx_buffer: [0; SERIAL_TX_BUFFER_SIZE as usize],
            _rx_buffer: [0; SERIAL_RX_BUFFER_SIZE as usize],
        }
    }

    pub fn begin(&self, baud: u32) {
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

    fn write(&mut self, c: u8) -> usize {
        self.written = true;

        if self._tx_buffer_head.read() == self._tx_buffer_tail.read() && UCSRA.udre0.is_set() {
            atomic_block!({
                UDR.write(c);
                UCSRA.update(|x| (x & ((1 << U2X0) | (1 << MPCM0))) | (1 << TXC0));
            });
            return 1;
        }
        let i = (self._tx_buffer_head.read() + 1) % SERIAL_TX_BUFFER_SIZE;

        while i == self._tx_buffer_tail.read() {
            if SREG.sreg_i.is_clear() {
                if UCSRA.udre0.is_set() {
                    self.tx_udr_empty_irq();
                }
            } else {
                // nop
            }
        }

        self._tx_buffer[self._tx_buffer_head.read() as usize] = c;

        atomic_block!({
            self._tx_buffer_head.write(i);
            UCSRB.udrie0.set();
        });

        1
    }

    fn flush(&mut self) {
        if !self.written {
            return;
        }

        while UCSRB.udrie0.is_set() || UCSRA.txc0.is_clear() {
            if SREG.sreg_i.is_clear() && UCSRB.udrie0.is_set() && UCSRA.udre0.is_set() {
                self.tx_udr_empty_irq();
            }
        }
    }

    pub(super) fn tx_udr_empty_irq(&mut self) {
        let c = self._tx_buffer[self._tx_buffer_tail.read() as usize];
        self._tx_buffer_tail
            .update(|x| (x + 1) % SERIAL_TX_BUFFER_SIZE);

        UDR.write(c);

        UCSRA.update(|x| (x & ((1 << U2X0) | (1 << MPCM0))) | (1 << TXC0));

        if self._tx_buffer_head.read() == self._tx_buffer_tail.read() {
            UCSRB.udrie0.clear();
        }
    }

    pub fn available_for_write(&self) -> u8 {
        let head;
        let tail;

        atomic_block!({
            head = self._rx_buffer_head.read();
            tail = self._tx_buffer_tail.read();
        });

        if head >= tail {
            SERIAL_TX_BUFFER_SIZE - 1 - head + tail
        } else {
            tail - head - 1
        }
    }
}

#[derive(Debug)]
pub struct HardwareSerial {}

impl HardwareSerial {
    #[inline]
    pub fn begin(baud: u32) -> Self {
        unsafe { HARDWARE_SERIAL.begin(baud) };
        Self {}
    }
}

impl Print for HardwareSerial {
    fn write_c(&mut self, c: u8) -> usize {
        unsafe { HARDWARE_SERIAL.write(c) }
    }

    fn write_buffer(&mut self, buffer: &[u8]) -> usize {
        let mut n = 0;

        for &c in buffer {
            n += self.write_c(c);
        }

        n
    }

    fn flush(&mut self) {
        unsafe { HARDWARE_SERIAL.flush() };
    }

    fn available_for_write(&self) -> usize {
        unsafe { HARDWARE_SERIAL.available_for_write() as usize }
    }
}

// impl ufmt::uWrite for HardwareSerial {
//     type Error = &'static str;

//     fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
//         self.write_c(c as u8);
//         Ok(())
//     }

//     fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
//         for &c in s.as_bytes() {
//             self.write_c(c);
//         }
//         Ok(())
//     }
// }
