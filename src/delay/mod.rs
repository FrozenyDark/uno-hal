mod delay_ms;
mod delay_us;
mod interrupt;
mod millis;

use crate::delay::{delay_ms::DelayMs, delay_us::DelayUs};
pub use millis::{get_ms, get_us};
use uno_hal_peripherals::timers::Timer0;

pub fn clock_ms_bg<F: FnMut()>(timer: &Timer0, mut ms: u32, mut f: F) {
    let mut start = get_us(timer);

    while ms > 0 {
        f();
        while ms > 0 && (get_us(timer) - start) >= 1000 {
            ms -= 1;
            start += 1000;
        }
    }
}

#[inline]
pub fn clock_ms(timer: &Timer0, ms: u32) {
    clock_ms_bg(timer, ms, || {});
}

#[inline]
pub fn delay_ms(ms: u32) {
    DelayMs::delay(ms);
}

#[inline]
pub fn delay_us(us: u32) {
    DelayUs::<u32>::delay(us);
}
