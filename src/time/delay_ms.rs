use crate::time::{delay_us::DelayUs, micros};

const MICROS_PER_MILLI: u32 = 1000;
const MAX_MILLIS: u32 = u32::MAX / MICROS_PER_MILLI;

pub struct DelayMs {}

impl DelayMs {
    #[allow(dead_code)]
    pub fn delay_legacy(mut ms: u32) {
        let mut start = micros();

        while ms > 0 {
            // yield
            while ms > 0 && (micros() - start) >= 1000 {
                ms -= 1;
                start += 1000;
            }
        }
    }

    pub fn delay(mut ms: u32) {
        while ms > MAX_MILLIS {
            ms -= MAX_MILLIS;
            DelayUs::<u32>::delay(MAX_MILLIS * MICROS_PER_MILLI);
        }

        DelayUs::<u32>::delay(ms * MICROS_PER_MILLI);
    }
}
