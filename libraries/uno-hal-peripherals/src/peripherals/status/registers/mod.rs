mod bits;

use crate::{addr::RegRW, bit::Bit};
pub use bits::*;

/// AVR Status Register
pub struct Sreg;

impl Sreg {
    pub const REG: RegRW<u8> = RegRW::new_io8::<0x3F>();

    pub const fn reg(self) -> RegRW<u8> {
        Self::REG
    }

    pub const fn bits(self) -> SregBits {
        SregBits {
            sreg_c: Bit::new(Self.reg()),
            sreg_z: Bit::new(Self.reg()),
            sreg_n: Bit::new(Self.reg()),
            sreg_v: Bit::new(Self.reg()),
            sreg_s: Bit::new(Self.reg()),
            sreg_h: Bit::new(Self.reg()),
            sreg_t: Bit::new(Self.reg()),
            sreg_i: Bit::new(Self.reg()),
        }
    }
}
