use core::marker::PhantomData;

use crate::{
    addr::{RegRO, RegRW},
    init_register,
};

/// USART I/O Data Register
pub struct Udr0(PhantomData<*const ()>);

init_register!(Udr0: RegRW<u8> = new_mem8(0xC6));
