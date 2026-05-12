use core::marker::PhantomData;

use crate::{
    addr::{RegRO, RegRW},
    bit::Bit,
    init_register,
};

/// Timer/Counter Control Register A
pub struct Tccr0A {
    /// Waveform Generation Mode Bit
    pub wgm00: Bit<RegRW<u8>, 0>,
    /// Waveform Generation Mode Bit
    pub wgm01: Bit<RegRW<u8>, 1>,
    /// Compare Match Output B Mode Bit
    pub com0b0: Bit<RegRW<u8>, 4>,
    /// Compare Match Output B Mode Bit
    pub com0b1: Bit<RegRW<u8>, 5>,
    /// Compare Match Output A Mode Bit
    pub com0a0: Bit<RegRW<u8>, 6>,
    /// Compare Match Output A Mode Bit
    pub com0a1: Bit<RegRW<u8>, 7>,
    _p: PhantomData<*const ()>,
}

/// Timer/Counter Control Register B
pub struct Tccr0B {
    /// Clock Select Bit
    pub cs00: Bit<RegRW<u8>, 0>,
    /// Clock Select Bit
    pub cs01: Bit<RegRW<u8>, 1>,
    /// Clock Select Bit
    pub cs02: Bit<RegRW<u8>, 2>,
    /// Waveform Generation Mode Bit
    pub wgm02: Bit<RegRW<u8>, 3>,
    /// Force Output Compare B
    pub foc0b: Bit<RegRW<u8>, 6>,
    /// Force Output Compare A
    pub foc0a: Bit<RegRW<u8>, 7>,
    _p: PhantomData<*const ()>,
}

/// Timer/Counter Control Register A
pub struct Tccr1A {
    /// Waveform Generation Mode Bit
    pub wgm10: Bit<RegRW<u8>, 0>,
    /// Waveform Generation Mode Bit
    pub wgm11: Bit<RegRW<u8>, 1>,
    /// Compare Match Output B Mode Bit
    pub com1b0: Bit<RegRW<u8>, 4>,
    /// Compare Match Output B Mode Bit
    pub com1b1: Bit<RegRW<u8>, 5>,
    /// Compare Match Output A Mode Bit
    pub com1a0: Bit<RegRW<u8>, 6>,
    /// Compare Match Output A Mode Bit
    pub com1a1: Bit<RegRW<u8>, 7>,
    _p: PhantomData<*const ()>,
}

/// Timer/Counter Control Register B
pub struct Tccr1B {
    /// Clock Select Bit
    pub cs10: Bit<RegRW<u8>, 0>,
    /// Clock Select Bit
    pub cs11: Bit<RegRW<u8>, 1>,
    /// Clock Select Bit
    pub cs12: Bit<RegRW<u8>, 2>,
    /// Waveform Generation Mode Bit
    pub wgm12: Bit<RegRW<u8>, 3>,
    /// Waveform Generation Mode Bit
    pub wgm13: Bit<RegRW<u8>, 4>,
    /// Input Capture Edge Select
    pub ices1: Bit<RegRW<u8>, 6>,
    /// Input Capture Noise Canceler
    pub icnc1: Bit<RegRW<u8>, 7>,
    _p: PhantomData<*const ()>,
}

/// Timer/Counter Control Register A
pub struct Tccr2A {
    /// Waveform Generation Mode Bit
    pub wgm20: Bit<RegRW<u8>, 0>,
    /// Waveform Generation Mode Bit
    pub wgm21: Bit<RegRW<u8>, 1>,
    /// Compare Match Output B Mode Bit
    pub com2b0: Bit<RegRW<u8>, 4>,
    /// Compare Match Output B Mode Bit
    pub com2b1: Bit<RegRW<u8>, 5>,
    /// Compare Match Output A Mode Bit
    pub com2a0: Bit<RegRW<u8>, 6>,
    /// Compare Match Output A Mode Bit
    pub com2a1: Bit<RegRW<u8>, 7>,
    _p: PhantomData<*const ()>,
}

/// Timer/Counter Control Register B
pub struct Tccr2B {
    /// Clock Select Bit
    pub cs20: Bit<RegRW<u8>, 0>,
    /// Clock Select Bit
    pub cs21: Bit<RegRW<u8>, 1>,
    /// Clock Select Bit
    pub cs22: Bit<RegRW<u8>, 2>,
    /// Waveform Generation Mode Bit
    pub wgm22: Bit<RegRW<u8>, 3>,
    /// Force Output Compare B
    pub foc2b: Bit<RegRW<u8>, 6>,
    /// Force Output Compare A
    pub foc2a: Bit<RegRW<u8>, 7>,
    _p: PhantomData<*const ()>,
}

init_register!(
    Tccr0A: RegRW<u8> = new_io8(0x24) {
        wgm00,
        wgm01,
        com0b0,
        com0b1,
        com0a0,
        com0a1,
    }
);

init_register!(
    Tccr0B: RegRW<u8> = new_io8(0x25) {
        cs00,
        cs01,
        cs02,
        wgm02,
        foc0b,
        foc0a,
    }
);

init_register!(
    Tccr1A: RegRW<u8> = new_mem8(0x80) {
        wgm10,
        wgm11,
        com1b0,
        com1b1,
        com1a0,
        com1a1,
    }
);

init_register!(
    Tccr1B: RegRW<u8> = new_mem8(0x81) {
        cs10,
        cs11,
        cs12,
        wgm12,
        wgm13,
        ices1,
        icnc1,
    }
);

init_register!(
    Tccr2A: RegRW<u8> = new_mem8(0xB0) {
        wgm20,
        wgm21,
        com2b0,
        com2b1,
        com2a0,
        com2a1,
    }
);

init_register!(
    Tccr2B: RegRW<u8> = new_mem8(0xB1) {
        cs20,
        cs21,
        cs22,
        wgm22,
        foc2b,
        foc2a,
    }
);
