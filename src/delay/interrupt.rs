use uno_hal_peripherals::F_CPU;

pub const CYCLES_PER_US: u32 = F_CPU / 1000000u32;

pub const fn cycles_to_us(a: u32) -> u32 {
    a / CYCLES_PER_US
}

pub const US_PER_TIMER0_OVERFLOW: u32 = cycles_to_us(64 * 256);
pub const MS_INC: u32 = US_PER_TIMER0_OVERFLOW / 1000;

pub const FRACT_INC: u32 = (US_PER_TIMER0_OVERFLOW % 1000) >> 3;
pub const FRACT_MAX: u32 = 1000 >> 3;

pub static mut TIMER0_OVERFLOW_COUNT: u32 = 0;
pub static mut TIMER0_MS: u32 = 0;
pub static mut TIMER0_FRACT: u32 = 0;

#[crate::interrupt(atmega328p)]
unsafe fn TIMER0_OVF() {
    let mut ms = TIMER0_MS.wrapping_add(MS_INC);
    let mut fract = TIMER0_FRACT + FRACT_INC;

    if fract >= FRACT_MAX {
        fract -= FRACT_MAX;
        ms = ms.wrapping_add(1);
    }

    TIMER0_FRACT = fract;
    TIMER0_MS = ms;
    TIMER0_OVERFLOW_COUNT = TIMER0_OVERFLOW_COUNT.wrapping_add(1);
}
