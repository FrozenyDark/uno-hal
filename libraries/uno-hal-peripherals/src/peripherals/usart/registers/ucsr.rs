use core::marker::PhantomData;

use crate::{
    addr::{RegRO, RegRW},
    bit::Bit,
    init_register,
};

/// USART Control and Status Register A
pub struct Ucsr0A {
    /// Multi-Processor Communication Mode
    pub mpcm0: Bit<RegRW<u8>, 0>,
    /// Double the USART Transmission Speed
    pub u2x0: Bit<RegRW<u8>, 1>,
    /// USART Parity Error
    pub upe0: Bit<RegRW<u8>, 2>,
    /// Data OverRun
    pub dor0: Bit<RegRW<u8>, 3>,
    /// Frame Error
    pub fe0: Bit<RegRW<u8>, 4>,
    /// USART Data Register Empty
    pub udre0: Bit<RegRW<u8>, 5>,
    /// USART Transmit Complete
    pub txc0: Bit<RegRW<u8>, 6>,
    /// USART Receive Complete
    pub rxc0: Bit<RegRW<u8>, 7>,
    _p: PhantomData<*const ()>,
}

/// USART Control and Status Register B
pub struct Ucsr0B {
    /// Transmit Data Bit 8
    pub txb80: Bit<RegRW<u8>, 0>,
    /// Receive Data Bit 8
    pub rxb80: Bit<RegRW<u8>, 1>,
    /// Character Size Bit
    pub ucsz02: Bit<RegRW<u8>, 2>,
    /// Transmitter Enable
    pub txen0: Bit<RegRW<u8>, 3>,
    /// Receiver Enable
    pub rxen0: Bit<RegRW<u8>, 4>,
    /// USART Data Register Empty Interrupt Enable
    pub udrie0: Bit<RegRW<u8>, 5>,
    /// TX Complete Interupt Enable
    pub txcie0: Bit<RegRW<u8>, 6>,
    /// RX Complete Interupt Enable
    pub rxcie0: Bit<RegRW<u8>, 7>,
    _p: PhantomData<*const ()>,
}

/// USART Control and Status Register C
pub struct Ucsr0C {
    /// Clock Polarity
    pub ucpol0: Bit<RegRW<u8>, 0>,
    /// Character Size Bit
    pub ucsz00: Bit<RegRW<u8>, 1>,
    /// Character Size Bit
    pub ucsz01: Bit<RegRW<u8>, 2>,
    /// Stop Bit Select
    pub usbs0: Bit<RegRW<u8>, 3>,
    /// USART Parity Mode Bit
    pub upm00: Bit<RegRW<u8>, 4>,
    /// USART Parity Mode Bit
    pub upm01: Bit<RegRW<u8>, 5>,
    /// USART Mode Select Bit
    pub umsel00: Bit<RegRW<u8>, 6>,
    /// USART Mode Select Bit
    pub umsel01: Bit<RegRW<u8>, 7>,
    _p: PhantomData<*const ()>,
}

init_register!(
    Ucsr0A: RegRW<u8> = new_mem8(0xC0) {
        mpcm0,
        u2x0,
        upe0,
        dor0,
        fe0,
        udre0,
        txc0,
        rxc0,
    }
);

init_register!(
    Ucsr0B: RegRW<u8> = new_mem8(0xC1) {
        txb80,
        rxb80,
        ucsz02,
        txen0,
        rxen0,
        udrie0,
        txcie0,
        rxcie0,
    }
);

init_register!(
    Ucsr0C: RegRW<u8> = new_mem8(0xC2) {
        ucpol0,
        ucsz00,
        ucsz01,
        usbs0,
        upm00,
        upm01,
        umsel00,
        umsel01,
    }
);
