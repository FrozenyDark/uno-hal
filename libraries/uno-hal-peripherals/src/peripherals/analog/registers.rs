use crate::{
    init_bits, init_register,
    register::{BitRO, BitRW, RegRO, RegRW},
};

init_bits! {
    AdcsraBits {
        #[doc = "ADC Prescaler Select Bit"]
        ADPS0 = 0,
        #[doc = "ADC Prescaler Select Bit"]
        ADPS1 = 1,
        #[doc = "ADC Prescaler Select Bit"]
        ADPS2 = 2,
        #[doc = "ADC Interrupt Enable"]
        ADIE = 3,
        #[doc = "ADC Interrupt Flag"]
        ADIF = 4,
        #[doc = "ADC Auto Trigger Enable"]
        ADATE = 5,
        #[doc = "ADC Start Conversion"]
        ADSC = 6,
        #[doc = "ADC Enable"]
        ADEN = 7,
    }
}

init_bits! {
    AdmuxBits {
        #[doc = "Analog Channel Selection Bit"]
        MUX0 = 0,
        #[doc = "Analog Channel Selection Bit"]
        MUX1 = 1,
        #[doc = "Analog Channel Selection Bit"]
        MUX2 = 2,
        #[doc = "Analog Channel Selection Bit"]
        MUX3 = 3,
        #[doc = "ADC Left Adjust Result"]
        ADLAR = 5,
        #[doc = "Reference Selection Bit"]
        REFS0 = 6,
        #[doc = "Reference Selection Bit"]
        REFS1 = 7,
    }
}

init_register! {
    #[doc = "ADC Data Register"]
    Adc: RegRO<u16> = new_mem16(0x78);
}

init_register! {
    #[doc = "ADC Control and Status Register A"]
    Adcsra: RegRW<u8> = new_mem8(0x7A) + AdcsraBits;
}

init_register! {
    #[doc = "ADC Multiplexer Selection Register"]
    Admux: RegRW<u8> = new_mem8(0x7C) + AdmuxBits;
}
