use crate::registers::*;

crate::INIT_REG!(ADC: mem16(0x78));

crate::INIT_REG!(
    ADCL: mem8(0x78),
    adcl0 = 0,
    adcl1 = 1,
    adcl2 = 2,
    adcl3 = 3,
    adcl4 = 4,
    adcl5 = 5,
    adcl6 = 6,
    adcl7 = 7
);

crate::INIT_REG!(
    ADCH: mem8(0x79),
    adch0 = 0,
    adch1 = 1,
    adch2 = 2,
    adch3 = 3,
    adch4 = 4,
    adch5 = 5,
    adch6 = 6,
    adch7 = 7
);

crate::INIT_REG!(
    ADCSRA: mem8(0x7A),
    adps0 = 0,
    adps1 = 1,
    adps2 = 2,
    adie = 3,
    adif = 4,
    adate = 5,
    adsc = 6,
    aden = 7
);

crate::INIT_REG!(
    ADMUX: mem8(0x7C),
    mux0 = 0,
    mux1 = 1,
    mux2 = 2,
    mux3 = 3,
    adlar = 5,
    refs0 = 6,
    refs1 = 7
);
