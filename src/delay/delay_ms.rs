use crate::delay::delay_us::DelayUs;

const US_PER_MS: u32 = 1000;
const MAX_MS: u32 = u32::MAX / US_PER_MS;

pub struct DelayMs {}

impl DelayMs {
    pub fn delay(mut ms: u32) {
        while ms > MAX_MS {
            ms -= MAX_MS;
            DelayUs::<u32>::delay(MAX_MS * US_PER_MS);
        }

        DelayUs::<u32>::delay(ms * US_PER_MS);
    }
}
