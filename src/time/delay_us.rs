use core::marker::PhantomData;

use crate::time::delay_loop::_delay_loop_2;

pub struct DelayUs<T> {
    _marker: PhantomData<T>,
}

impl DelayUs<u16> {
    pub fn delay(mut us: u16) {
        if us <= 1 {
            return;
        } // 3 cycles (4 when true)

        us <<= 2; // x4 us, 4 cycles
        us -= 5; // 2 cycles

        unsafe { _delay_loop_2(us) };
    }
}

impl DelayUs<u32> {
    pub fn delay(us: u32) {
        let iters = us >> 12;
        let mut i = 0;

        while i < iters {
            DelayUs::<u16>::delay(0xFFF);
            i += 1;
        }

        DelayUs::<u16>::delay((us & 0xFFF) as u16);
    }
}
