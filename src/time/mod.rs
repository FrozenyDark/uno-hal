pub mod delay_loop;
pub mod delay_ms;
pub mod delay_us;
mod millis;
mod timer;

pub use millis::*;

use crate::time::{delay_ms::DelayMs, delay_us::DelayUs};

#[inline]
pub fn delay_legacy(ms: u32) {
    DelayMs::delay_legacy(ms);
}

#[inline]
pub fn delay(ms: u32) {
    DelayMs::delay(ms);
}

#[inline]
pub fn delay_us(us: u32) {
    DelayUs::<u32>::delay(us);
}
