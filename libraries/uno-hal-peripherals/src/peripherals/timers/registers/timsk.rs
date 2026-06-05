use crate::{
    init_bits, init_register,
    register::{BitRO, BitRW, RegRO, RegRW},
};

init_bits! {
    Timsk0Bits {
        #[doc = "Timer/Counter Output Overflow Interrupt Enable"]
        TOIE0 = 0,
        #[doc = "Timer/Counter Output Compare Match A Interrupt Enable"]
        OCIE0A = 1,
        #[doc = "Timer/Counter Output Compare Match B Interrupt Enable"]
        OCIE0B = 2,
    }
}

init_bits! {
    Timsk1Bits {
        #[doc = "Timer/Counter Output Overflow Interrupt Enable"]
        TOIE1 = 0,
        #[doc = "Timer/Counter Output Compare Match A Interrupt Enable"]
        OCIE1A = 1,
        #[doc = "Timer/Counter Output Compare Match B Interrupt Enable"]
        OCIE1B = 2,
    }
}

init_bits! {
    Timsk2Bits {
        #[doc = "Timer/Counter Output Overflow Interrupt Enable"]
        TOIE2 = 0,
        #[doc = "Timer/Counter Output Compare Match A Interrupt Enable"]
        OCIE2A = 1,
        #[doc = "Timer/Counter Output Compare Match B Interrupt Enable"]
        OCIE2B = 2,
    }
}

init_register! {
    #[doc = "Timer/Counter 0 Interrupt Mask"]
    Timsk0: RegRW<u8> = new_mem8(0x6E) + Timsk0Bits;
}

init_register! {
    #[doc = "Timer/Counter 1 Interrupt Mask"]
    Timsk1: RegRW<u8> = new_mem8(0x6F) + Timsk1Bits;
}

init_register! {
    #[doc = "Timer/Counter 2 Interrupt Mask"]
    Timsk2: RegRW<u8> = new_mem8(0x70) + Timsk2Bits;
}
