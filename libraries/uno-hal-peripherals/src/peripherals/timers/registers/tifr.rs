use crate::{
    init_bits, init_register,
    register::{BitRO, BitRW, RegRO, RegRW},
};

init_bits! {
    Tifr0Bits {
        #[doc = "Timer/Counter Overflow Flag"]
        TOV0 = 0,
        #[doc = "Timer/Counter Output Compare A Match Flag"]
        OCF0A = 1,
        #[doc = "Timer/Counter Output Compare B Match Flag"]
        OCF0B = 2,
    }
}

init_bits! {
    Tifr1Bits {
        #[doc = "Timer/Counter Overflow Flag"]
        TOV1 = 0,
        #[doc = "Timer/Counter Output Compare A Match Flag"]
        OCF1A = 1,
        #[doc = "Timer/Counter Output Compare B Match Flag"]
        OCF1B = 2,
        #[doc = "Timer/Counter Input Capture Flag"]
        ICF1 = 5,
    }
}

init_bits! {
    Tifr2Bits {
        #[doc = "Timer/Counter Overflow Flag"]
        TOV2 = 0,
        #[doc = "Timer/Counter Output Compare A Match Flag"]
        OCF2A = 1,
        #[doc = "Timer/Counter Output Compare B Match Flag"]
        OCF2B = 2,
    }
}

init_register! {
    #[doc = "Timer/Counter 0 Interrupt Flag"]
    Tifr0: RegRW<u8> = new_io8(0x15) + Tifr0Bits;
}

init_register! {
    #[doc = "Timer/Counter 1 Interrupt Flag"]
    Tifr1: RegRW<u8> = new_io8(0x16) + Tifr1Bits;
}

init_register! {
    #[doc = "Timer/Counter 2 Interrupt Flag"]
    Tifr2: RegRW<u8> = new_io8(0x17) + Tifr2Bits;
}
