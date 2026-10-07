use uno_hal_peripherals::{status::CriticalSection, usart::Usart0};

use crate::peripherals::usart::{
    worker::{rx_handler, tx_handler},
    RX_BUFFER, TX_BUFFER,
};

#[crate::interrupt(atmega328p)]
unsafe fn USART_UDRE() {
    let cs = CriticalSection::new();
    let mut tx = TX_BUFFER.borrow_ref_mut(cs);
    tx_handler(&mut Usart0::take(), &mut tx);
}

#[crate::interrupt(atmega328p)]
unsafe fn USART_RX() {
    let cs = CriticalSection::new();
    let mut rx = RX_BUFFER.borrow_ref_mut(cs);
    rx_handler(&Usart0::take(), &mut rx);
}
