mod interrupt;
mod worker;
pub mod writable;

use crate::peripherals::usart::{
    worker::{UsartWorker, USART_WORKER},
    writable::Writable,
};
use core::marker::PhantomData;
use uno_hal_peripherals::usart::Usart0;

pub struct HwSerial(PhantomData<*const ()>);

impl HwSerial {
    #[inline]
    pub fn new(usart: Usart0, baud: u32) -> Self {
        UsartWorker::create(usart);
        unsafe { USART_WORKER.as_mut().unwrap().begin(baud) };

        Self(PhantomData)
    }

    #[inline]
    pub fn read(&self) -> Option<u8> {
        unsafe { USART_WORKER.as_mut().unwrap().read() }
    }
}

impl Writable for HwSerial {
    #[inline]
    fn write_c(&mut self, c: u8) -> usize {
        unsafe { USART_WORKER.as_mut().unwrap().write(c) }
    }

    #[inline]
    fn flush(&mut self) {
        unsafe { USART_WORKER.as_mut().unwrap().flush() };
    }

    #[inline]
    fn available_for_write(&self) -> usize {
        unsafe { USART_WORKER.as_mut().unwrap().available_for_write() as usize }
    }
}
