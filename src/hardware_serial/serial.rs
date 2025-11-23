use crate::hardware_serial::{print::Print, serial_worker::SERIAL_WORKER};

#[derive(Debug)]
pub struct HardwareSerial {}

impl HardwareSerial {
    pub fn begin(baud: u32) -> Self {
        unsafe { SERIAL_WORKER.begin(baud) };
        Self {}
    }
}

impl Print for HardwareSerial {
    #[inline]
    fn write_c(&mut self, c: u8) -> usize {
        unsafe { SERIAL_WORKER.write(c) }
    }

    #[inline]
    fn flush(&mut self) {
        unsafe { SERIAL_WORKER.flush() };
    }

    #[inline]
    fn available_for_write(&self) -> usize {
        unsafe { SERIAL_WORKER.available_for_write() as usize }
    }
}
