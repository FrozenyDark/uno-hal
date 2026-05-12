use core::marker::PhantomData;

use crate::{
    addr::{RegRO, RegRW},
    init_register,
};

/// Output Compare Register A
pub struct Ocr0A(PhantomData<*const ()>);

/// Output Compare Register B
pub struct Ocr0B(PhantomData<*const ()>);

/// Output Compare Register A
pub struct Ocr1A(PhantomData<*const ()>);

/// Output Compare Register B
pub struct Ocr1B(PhantomData<*const ()>);

/// Output Compare Register A
pub struct Ocr2A(PhantomData<*const ()>);

/// Output Compare Register B
pub struct Ocr2B(PhantomData<*const ()>);

init_register!(Ocr0A: RegRW<u8> = new_io8(0x27));
init_register!(Ocr0B: RegRW<u8> = new_io8(0x28));

init_register!(Ocr1A: RegRW<u16> = new_mem16(0x88));
init_register!(Ocr1B: RegRW<u16> = new_mem16(0x8A));

init_register!(Ocr2A: RegRW<u8> = new_mem8(0xB3));
init_register!(Ocr2B: RegRW<u8> = new_mem8(0xB4));
