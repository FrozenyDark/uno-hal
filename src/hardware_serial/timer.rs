use crate::hardware_serial::serial_worker::SERIAL_WORKER;

#[uno_hal_macro::interrupt(atmega328p)]
unsafe fn USART_UDRE() {
    SERIAL_WORKER.tx_udr_empty_irq();
}
