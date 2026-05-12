use crate::{addr::RegRW, bit::Bit};

/// Status Register Bits
pub struct SregBits {
    /// Carry Flag
    pub sreg_c: Bit<RegRW<u8>, 0>,
    /// Zero Flag
    pub sreg_z: Bit<RegRW<u8>, 1>,
    /// Negative Flag
    pub sreg_n: Bit<RegRW<u8>, 2>,
    /// Two's complement Overflow Flag
    pub sreg_v: Bit<RegRW<u8>, 3>,
    /// Sign Bit, `s` = `n` XOR `v`
    pub sreg_s: Bit<RegRW<u8>, 4>,
    /// Half Carry Flag
    pub sreg_h: Bit<RegRW<u8>, 5>,
    /// Bit Copy Flag
    pub sreg_t: Bit<RegRW<u8>, 6>,
    /// Global Interrupt Enable
    pub sreg_i: Bit<RegRW<u8>, 7>,
}
