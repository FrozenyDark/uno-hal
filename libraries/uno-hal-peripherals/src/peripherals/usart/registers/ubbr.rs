use core::marker::PhantomData;

use crate::{
    addr::{RegRO, RegRW},
    init_register,
};

/// USART Baud Rate Registers
pub struct Ubbr0(PhantomData<*const ()>);

init_register!(Ubbr0: RegRW<u16> = new_mem16(0xC4));
