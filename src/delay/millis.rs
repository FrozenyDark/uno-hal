use crate::delay::interrupt::{CYCLES_PER_US, TIMER0_MS, TIMER0_OVERFLOW_COUNT};
use uno_hal_peripherals::{atomic_block, timers::Timer0};

#[inline]
pub fn get_ms() -> u32 {
    atomic_block! { unsafe { TIMER0_MS } }
}

pub fn get_us(timer: &Timer0) -> u32 {
    let mut m: u32;
    let t: u8;

    atomic_block! {
        m = unsafe { TIMER0_OVERFLOW_COUNT };
        t = timer.tcnt0.reg().read();

        if timer.tifr0.tov0.is_set() && (t < u8::MAX) {
            m += 1;
        }
    }

    ((m << 8) + t as u32) * (64 / CYCLES_PER_US)
}
