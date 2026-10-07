use core::cell::RefCell;

use uno_hal_peripherals::{status::Mutex, usart::Usart0};

use crate::common::bytebuffer::ByteBuffer;

const BUFFER_SIZE: usize = 16;

pub(super) static RX_BUFFER: Mutex<RefCell<ByteBuffer<BUFFER_SIZE>>> =
    Mutex::new(RefCell::new(ByteBuffer::new()));

pub(super) static TX_BUFFER: Mutex<RefCell<ByteBuffer<BUFFER_SIZE>>> =
    Mutex::new(RefCell::new(ByteBuffer::new()));

pub(super) fn tx_handler(usart: &mut Usart0, tx: &mut ByteBuffer<BUFFER_SIZE>) {
    let Some(byte) = tx.pop() else {
        usart.set_tx_interrupt(false);
        return;
    };

    usart.write_byte(byte);

    if tx.is_empty() {
        usart.set_tx_interrupt(false);
    }
}

pub(super) fn rx_handler(usart: &Usart0, rx: &mut ByteBuffer<BUFFER_SIZE>) {
    if usart.parity_error() {
        let _ = usart.read_byte();
    }

    rx.add(usart.read_byte());
}
