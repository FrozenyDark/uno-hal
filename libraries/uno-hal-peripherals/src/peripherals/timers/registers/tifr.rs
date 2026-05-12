use core::marker::PhantomData;

use crate::{
    addr::{RegRO, RegRW},
    bit::Bit,
    init_register,
};

/// Timer/Counter Interrupt Flag
pub struct Tifr0 {
    /// Timer/Counter Overflow Flag
    pub tov0: Bit<RegRW<u8>, 0>,
    /// Timer/Counter Output Compare A Match Flag
    pub ocf0a: Bit<RegRW<u8>, 1>,
    /// Timer/Counter Output Compare B Match Flag
    pub ocf0b: Bit<RegRW<u8>, 2>,
    _p: PhantomData<*const ()>,
}

/// Timer/Counter Interrupt Flag
pub struct Tifr1 {
    /// Timer/Counter Overflow Flag
    pub tov1: Bit<RegRW<u8>, 0>,
    /// Timer/Counter Output Compare A Match Flag
    pub ocf1a: Bit<RegRW<u8>, 1>,
    /// Timer/Counter Output Compare B Match Flag
    pub ocf1b: Bit<RegRW<u8>, 2>,
    /// Timer/Counter Input Capture Flag
    pub icf1: Bit<RegRW<u8>, 5>,
    _p: PhantomData<*const ()>,
}

/// Timer/Counter Interrupt Flag
pub struct Tifr2 {
    /// Timer/Counter Overflow Flag
    pub tov2: Bit<RegRW<u8>, 0>,
    /// Timer/Counter Output Compare A Match Flag
    pub ocf2a: Bit<RegRW<u8>, 1>,
    /// Timer/Counter Output Compare B Match Flag
    pub ocf2b: Bit<RegRW<u8>, 2>,
    _p: PhantomData<*const ()>,
}

init_register!(
    Tifr0: RegRW<u8> = new_io8(0x15) {
        tov0,
        ocf0a,
        ocf0b,
    }
);

init_register!(
    Tifr1: RegRW<u8> = new_io8(0x16) {
        tov1,
        ocf1a,
        ocf1b,
        icf1,
    }
);

init_register!(
    Tifr2: RegRW<u8> = new_io8(0x17) {
        tov2,
        ocf2a,
        ocf2b,
    }
);
