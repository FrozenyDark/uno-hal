use crate::hardware_serial::serial_worker::SERIAL_WORKER;

#[crate::interrupt(atmega328p)]
unsafe fn USART_UDRE() {
    SERIAL_WORKER.tx_udr_empty_irq();
}
