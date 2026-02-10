use super::timer::{clock_cycles_per_microsecond, TIMER0_MILLIS, TIMER0_OVERFLOW_COUNT};
use uno_hal_registers::{
    atomic_block,
    registers::timers::{TCNT0, TIFR0},
};

#[inline]
pub fn millis() -> u32 {
    atomic_block! { unsafe { TIMER0_MILLIS } }
}

pub fn micros() -> u32 {
    let mut m: u32;
    let t: u8;

    atomic_block!({
        m = unsafe { TIMER0_OVERFLOW_COUNT };
        t = TCNT0.read();

        if TIFR0.tov0.is_set() && (t < 255) {
            m += 1;
        }
    });

    ((m << 8) + t as u32) * (64 / clock_cycles_per_microsecond())
}
