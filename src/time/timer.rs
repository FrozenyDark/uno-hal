use crate::{registers::F_CPU, TIMER0_OVF_vect};

pub const fn clock_cycles_per_microsecond() -> u32 {
    F_CPU / 1000000u32
}

pub const fn clock_cycles_to_microseconds(a: u32) -> u32 {
    a / clock_cycles_per_microsecond()
}

pub const MICROSECONDS_PER_TIMER0_OVERFLOW: u32 = clock_cycles_to_microseconds(64 * 256);
pub const MILLIS_INC: u32 = MICROSECONDS_PER_TIMER0_OVERFLOW / 1000;

pub const FRACT_INC: u32 = (MICROSECONDS_PER_TIMER0_OVERFLOW % 1000) >> 3;
pub const FRACT_MAX: u32 = 1000 >> 3;

pub static mut TIMER0_OVERFLOW_COUNT: u32 = 0;
pub static mut TIMER0_MILLIS: u32 = 0;
pub static mut TIMER0_FRACT: u32 = 0;

#[allow(non_snake_case)]
#[export_name = TIMER0_OVF_vect!()]
extern "avr-interrupt" fn TIMER0_OVF() {
    let mut m = unsafe { TIMER0_MILLIS };
    let mut f = unsafe { TIMER0_FRACT };

    m += MILLIS_INC;
    f += FRACT_INC;
    if f >= FRACT_MAX {
        f -= FRACT_MAX;
        m += 1;
    }

    unsafe {
        TIMER0_FRACT = f;
        TIMER0_MILLIS = m;
        TIMER0_OVERFLOW_COUNT += 1;
    }
}
