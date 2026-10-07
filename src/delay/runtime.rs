use crate::delay::interrupt::{CYCLES_PER_US, TIMER0_COUNTER};
use uno_hal_peripherals::{
    register::RO,
    status::atomic_block,
    timers::{registers::tifr::Tifr0Bits, Timer0},
};

#[inline]
pub fn get_ms() -> u32 {
    atomic_block(|cs| TIMER0_COUNTER.borrow_ref(cs).ms)
}

#[inline(never)]
pub fn get_us(timer: &Timer0) -> u32 {
    let (m, t) = atomic_block(|cs| {
        let mut m = TIMER0_COUNTER.borrow_ref(cs).overflows;
        let t = timer.tcnt0.reg().read();

        if timer.tifr0.is_set_bit(Tifr0Bits::TOV0) && (t < u8::MAX) {
            m += 1;
        }

        (m, t)
    });

    ((m << 8) + t as u32) * (64 / CYCLES_PER_US)
}
