use core::marker::PhantomData;

use crate::{
    addr::{RegRO, RegRW},
    bit::Bit,
    init_register,
};

/// ADC Data Register
pub struct Adc(PhantomData<*const ()>);

/// ADC Control and Status Register A
pub struct Adcsra {
    /// ADC Prescaler Select Bit
    pub adps0: Bit<RegRW<u8>, 0>,
    /// ADC Prescaler Select Bit
    pub adps1: Bit<RegRW<u8>, 1>,
    /// ADC Prescaler Select Bit
    pub adps2: Bit<RegRW<u8>, 2>,
    /// ADC Interrupt Enable
    pub adie: Bit<RegRW<u8>, 3>,
    /// ADC Interrupt Flag
    pub adif: Bit<RegRW<u8>, 4>,
    /// ADC Auto Trigger Enable
    pub adate: Bit<RegRW<u8>, 5>,
    /// ADC Start Conversion
    pub adsc: Bit<RegRW<u8>, 6>,
    /// ADC Enable
    pub aden: Bit<RegRW<u8>, 7>,
    _p: PhantomData<*const ()>,
}

/// ADC Multiplexer Selection Register
pub struct Admux {
    /// Analog Channel Selection Bit
    pub mux0: Bit<RegRW<u8>, 0>,
    /// Analog Channel Selection Bit
    pub mux1: Bit<RegRW<u8>, 1>,
    /// Analog Channel Selection Bit
    pub mux2: Bit<RegRW<u8>, 2>,
    /// Analog Channel Selection Bit
    pub mux3: Bit<RegRW<u8>, 3>,
    /// ADC Left Adjust Result
    pub adlar: Bit<RegRW<u8>, 5>,
    /// Reference Selection Bit
    pub refs0: Bit<RegRW<u8>, 6>,
    /// Reference Selection Bit
    pub refs1: Bit<RegRW<u8>, 7>,
    _p: PhantomData<*const ()>,
}

init_register!(Adc: RegRO<u16> = new_mem16(0x78));

init_register!(
    Adcsra: RegRW<u8> = new_mem8(0x7A) {
        adps0,
        adps1,
        adps2,
        adie,
        adif,
        adate,
        adsc,
        aden,
    }
);

init_register!(
    Admux: RegRW<u8> = new_mem8(0x7C) {
        mux0,
        mux1,
        mux2,
        mux3,
        adlar,
        refs0,
        refs1,
    }
);
