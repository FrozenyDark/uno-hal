use crate::{hardware_serial::serial_worker::SERIAL_WORKER, USART_UDRE_vect};

#[allow(non_snake_case)]
#[export_name = USART_UDRE_vect!()]
unsafe extern "avr-interrupt" fn USART_UDRE() {
    SERIAL_WORKER.tx_udr_empty_irq();
}
