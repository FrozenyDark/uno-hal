use uno_hal_peripherals::usart::Usart0;

use crate::peripherals::usart::worker::USART_BUFFER;

#[crate::interrupt(atmega328p)]
unsafe fn USART_UDRE() {
    USART_BUFFER.tx_handler(&mut Usart0::take());
}

#[crate::interrupt(atmega328p)]
unsafe fn USART_RX() {
    USART_BUFFER.rx_handler(&mut Usart0::take());
}
