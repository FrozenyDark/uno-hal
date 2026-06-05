use crate::{
    init_bits, init_register,
    register::{BitRO, BitRW, RegRO, RegRW},
};

init_bits! {
    SregBits {
        #[doc = "Carry Flag"]
        C = 0,
        #[doc = "Zero Flag"]
        Z = 1,
        #[doc = "Negative Flag"]
        N = 2,
        #[doc = "Two's complement Overflow Flag"]
        V = 3,
        #[doc = "Sign Bit, `s` = `n` XOR `v`"]
        S = 4,
        #[doc = "Half Carry Flag"]
        H = 5,
        #[doc = "Bit Copy Flag"]
        T = 6,
        #[doc = "Global Interrupt Enable"]
        I = 7,
    }
}

init_register! {
    #[doc = "AVR Status Register"]
    Sreg: RegRW<u8> = new_io8(0x3F) + SregBits;
}
