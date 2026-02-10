use crate::registers::*;

crate::INIT_REG!(
    UCSR0A: mem8(0xC0),
    mpcm0 = 0,
    u2x0 = 1,
    upe0 = 2,
    dor0 = 3,
    fe0 = 4,
    udre0 = 5,
    txc0 = 6,
    rxc0 = 7
);

crate::INIT_REG!(
    UCSR0B: mem8(0xC1),
    txb80 = 0,
    rxb80 = 1,
    ucsz02 = 2,
    txen0 = 3,
    rxen0 = 4,
    udrie0 = 5,
    txcie0 = 6,
    rxcie0 = 7
);

crate::INIT_REG!(
    UCSR0C: mem8(0xC2),
    ucpol0 = 0,
    ucsz00 = 1,
    ucpha0 = 1,
    ucsz01 = 2,
    udord0 = 2,
    usbs0 = 3,
    upm00 = 4,
    upm01 = 5,
    umsel00 = 6,
    umsel01 = 7
);

crate::INIT_REG!(UBRR0: mem16(0xC4));

crate::INIT_REG!(
    UBRR0L: mem8(0xC4),
    ubrr0_0 = 0,
    ubrr0_1 = 1,
    ubrr0_2 = 2,
    ubrr0_3 = 3,
    ubrr0_4 = 4,
    ubrr0_5 = 5,
    ubrr0_6 = 6,
    ubrr0_7 = 7
);

crate::INIT_REG!(
    UBRR0H: mem8(0xC5),
    ubrr0_8 = 0,
    ubrr0_9 = 1,
    ubrr0_10 = 2,
    ubrr0_11 = 3
);

crate::INIT_REG!(
    UDR0: mem8(0xC6),
    udr0_0 = 0,
    udr0_1 = 1,
    udr0_2 = 2,
    udr0_3 = 3,
    udr0_4 = 4,
    udr0_5 = 5,
    udr0_6 = 6,
    udr0_7 = 7
);
