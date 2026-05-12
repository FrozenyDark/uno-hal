use core::marker::PhantomData;

use crate::{
    addr::{RegRO, RegRW},
    bit::Bit,
    init_register,
};

pub struct Timsk0 {
    /// Timer/Counter Output Overflow Interrupt Enable
    pub toie_: Bit<RegRW<u8>, 0>,
    /// Timer/Counter Output Compare Match A Interrupt Enable
    pub ocie_a: Bit<RegRW<u8>, 1>,
    /// Timer/Counter Output Compare Match B Interrupt Enable
    pub ocie_b: Bit<RegRW<u8>, 2>,
    _p: PhantomData<*const ()>,
}

pub struct Timsk1 {
    /// Timer/Counter Output Overflow Interrupt Enable
    pub toie_: Bit<RegRW<u8>, 0>,
    /// Timer/Counter Output Compare Match A Interrupt Enable
    pub ocie_a: Bit<RegRW<u8>, 1>,
    /// Timer/Counter Output Compare Match B Interrupt Enable
    pub ocie_b: Bit<RegRW<u8>, 2>,
    _p: PhantomData<*const ()>,
}

pub struct Timsk2 {
    /// Timer/Counter Output Overflow Interrupt Enable
    pub toie_: Bit<RegRW<u8>, 0>,
    /// Timer/Counter Output Compare Match A Interrupt Enable
    pub ocie_a: Bit<RegRW<u8>, 1>,
    /// Timer/Counter Output Compare Match B Interrupt Enable
    pub ocie_b: Bit<RegRW<u8>, 2>,
    _p: PhantomData<*const ()>,
}

init_register!(
    Timsk0: RegRW<u8> = new_mem8(0x6E) {
        toie_,
        ocie_a,
        ocie_b
    }
);

init_register!(
    Timsk1: RegRW<u8> = new_mem8(0x6F) {
        toie_,
        ocie_a,
        ocie_b
    }
);

init_register!(
    Timsk2: RegRW<u8> = new_mem8(0x70) {
        toie_,
        ocie_a,
        ocie_b
    }
);
