use core::marker::PhantomData;

use crate::{
    addr::{RegRO, RegRW},
    init_register,
};

/// Timer/Counter Register
pub struct Tcnt0(PhantomData<*const ()>);

/// Timer/Counter Register
pub struct Tcnt1(PhantomData<*const ()>);

/// Timer/Counter Register
pub struct Tcnt2(PhantomData<*const ()>);

init_register!(Tcnt0: RegRW<u8> = new_io8(0x26));
init_register!(Tcnt1: RegRW<u16> = new_mem16(0x84));
init_register!(Tcnt2: RegRW<u8> = new_mem8(0xB2));
