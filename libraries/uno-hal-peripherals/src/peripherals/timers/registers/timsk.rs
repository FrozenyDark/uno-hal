use core::marker::PhantomData;

use crate::{
    addr::{RegRO, RegRW},
    bit::Bit,
    init_register,
};

pub struct Timsk0 {
    /// Timer/Counter Output Overflow Interrupt Enable
    pub toie0: Bit<RegRW<u8>, 0>,
    /// Timer/Counter Output Compare Match A Interrupt Enable
    pub ocie0a: Bit<RegRW<u8>, 1>,
    /// Timer/Counter Output Compare Match B Interrupt Enable
    pub ocie0b: Bit<RegRW<u8>, 2>,
    _p: PhantomData<*const ()>,
}

pub struct Timsk1 {
    /// Timer/Counter Output Overflow Interrupt Enable
    pub toie1: Bit<RegRW<u8>, 0>,
    /// Timer/Counter Output Compare Match A Interrupt Enable
    pub ocie1a: Bit<RegRW<u8>, 1>,
    /// Timer/Counter Output Compare Match B Interrupt Enable
    pub ocie1b: Bit<RegRW<u8>, 2>,
    _p: PhantomData<*const ()>,
}

pub struct Timsk2 {
    /// Timer/Counter Output Overflow Interrupt Enable
    pub toie2: Bit<RegRW<u8>, 0>,
    /// Timer/Counter Output Compare Match A Interrupt Enable
    pub ocie2a: Bit<RegRW<u8>, 1>,
    /// Timer/Counter Output Compare Match B Interrupt Enable
    pub ocie2b: Bit<RegRW<u8>, 2>,
    _p: PhantomData<*const ()>,
}

init_register!(
    Timsk0: RegRW<u8> = new_mem8(0x6E) {
        toie0,
        ocie0a,
        ocie0b
    }
);

init_register!(
    Timsk1: RegRW<u8> = new_mem8(0x6F) {
        toie1,
        ocie1a,
        ocie1b
    }
);

init_register!(
    Timsk2: RegRW<u8> = new_mem8(0x70) {
        toie2,
        ocie2a,
        ocie2b
    }
);
