use crate::peripherals::usart::worker::USART_WORKER;

#[crate::interrupt(atmega328p)]
unsafe fn USART_UDRE() {
    if let Some(worker) = &mut USART_WORKER {
        worker.tx_interrupt();
    }
}

#[crate::interrupt(atmega328p)]
unsafe fn USART_RX() {
    if let Some(worker) = &mut USART_WORKER {
        worker.rx_interrupt();
    }
}
