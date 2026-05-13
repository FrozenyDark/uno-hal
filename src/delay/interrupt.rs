use uno_hal_peripherals::F_CPU;

pub(super) const CYCLES_PER_US: u32 = F_CPU / 1_000_000;

const fn cycles_to_us(a: u32) -> u32 {
    a / CYCLES_PER_US
}

/// TCNT0 max (u8::MAX) * Selected Prescaler (64)
const US_PER_TIMER0_OVERFLOW: u32 = cycles_to_us(u8::MAX as u32 * 64);
const MS_INC: u32 = US_PER_TIMER0_OVERFLOW / 1000;

const FRACT_INC: u8 = ((US_PER_TIMER0_OVERFLOW % 1000) >> 3) as u8;
const FRACT_MAX: u8 = (1000 >> 3) as u8;

pub(super) static mut TIMER0_COUNTER: TimerCounter = TimerCounter::new();

pub(super) struct TimerCounter {
    pub overflows: u32,
    pub ms: u32,
    fract: u8,
}

impl TimerCounter {
    const fn new() -> Self {
        Self {
            overflows: 0,
            ms: 0,
            fract: 0,
        }
    }

    #[inline]
    fn add_overflow(&mut self) {
        let mut ms = self.ms.wrapping_add(MS_INC);
        let mut fract = self.fract + FRACT_INC;

        if fract >= FRACT_MAX {
            fract -= FRACT_MAX;
            ms = ms.wrapping_add(1);
        }

        self.ms = ms;
        self.fract = fract;
        self.overflows += 1;
    }
}

#[crate::interrupt(atmega328p)]
unsafe fn TIMER0_OVF() {
    TIMER0_COUNTER.add_overflow();
    // let mut ms = TIMER0_MS.wrapping_add(MS_INC);
    // let mut fract = TIMER0_FRACT + FRACT_INC;

    // if fract >= FRACT_MAX {
    //     fract -= FRACT_MAX;
    //     ms = ms.wrapping_add(1);
    // }

    // TIMER0_FRACT = fract;
    // TIMER0_MS = ms;
    // TIMER0_OVERFLOW_COUNT = TIMER0_OVERFLOW_COUNT.wrapping_add(1);
}
