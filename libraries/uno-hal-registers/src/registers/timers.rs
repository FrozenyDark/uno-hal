use crate::registers::*;

crate::INIT_REG!(
    TCCR0A: io8(0x24),
    wgm00 = 0,
    wgm01 = 1,
    com0b0 = 4,
    com0b1 = 5,
    com0a0 = 6,
    com0a1 = 7
);

crate::INIT_REG!(
    TCCR0B: io8(0x25),
    cs00 = 0,
    cs01 = 1,
    cs02 = 2,
    wgm02 = 3,
    foc0b = 6,
    foc0a = 7
);

crate::INIT_REG!(
    TCCR1A: mem8(0x80),
    wgm10 = 0,
    wgm11 = 1,
    com1b0 = 4,
    com1b1 = 5,
    com1a0 = 6,
    com1a1 = 7
);

crate::INIT_REG!(
    TCCR1B: mem8(0x81),
    cs10 = 0,
    cs11 = 1,
    cs12 = 2,
    wgm12 = 3,
    wgm13 = 4,
    ices1 = 6,
    icnc1 = 7
);

crate::INIT_REG!(
    TCCR2A: mem8(0xB0),
    wgm20 = 0,
    wgm21 = 1,
    com2b0 = 4,
    com2b1 = 5,
    com2a0 = 6,
    com2a1 = 7
);

crate::INIT_REG!(
    TCCR2B: mem8(0xB1),
    cs20 = 0,
    cs21 = 1,
    cs22 = 2,
    wgm22 = 3,
    foc2b = 6,
    foc2a = 7
);

crate::INIT_REG!(
    TCNT0: io8(0x26),
    tcnt0_0 = 0,
    tcnt0_1 = 1,
    tcnt0_2 = 2,
    tcnt0_3 = 3,
    tcnt0_4 = 4,
    tcnt0_5 = 5,
    tcnt0_6 = 6,
    tcnt0_7 = 7
);

crate::INIT_REG!(TCNT1: mem16(0x84));

crate::INIT_REG!(
    TCNT1L: mem8(0x84),
    tcnt1l0 = 0,
    tcnt1l1 = 1,
    tcnt1l2 = 2,
    tcnt1l3 = 3,
    tcnt1l4 = 4,
    tcnt1l5 = 5,
    tcnt1l6 = 6,
    tcnt1l7 = 7
);

crate::INIT_REG!(
    TCNT1H: mem8(0x85),
    tcnt1h0 = 0,
    tcnt1h1 = 1,
    tcnt1h2 = 2,
    tcnt1h3 = 3,
    tcnt1h4 = 4,
    tcnt1h5 = 5,
    tcnt1h6 = 6,
    tcnt1h7 = 7
);

crate::INIT_REG!(
    TCNT2: mem8(0xB2),
    tcnt2_0 = 0,
    tcnt2_1 = 1,
    tcnt2_2 = 2,
    tcnt2_3 = 3,
    tcnt2_4 = 4,
    tcnt2_5 = 5,
    tcnt2_6 = 6,
    tcnt2_7 = 7
);

crate::INIT_REG!(
    TIFR0: io8(0x15),
    tov0 = 0,
    ocf0a = 1,
    ocf0b = 2
);

crate::INIT_REG!(
    TIFR1: io8(0x16),
    tov1 = 0,
    ocf1a = 1,
    ocf1b = 2,
    icf1 = 5
);

crate::INIT_REG!(
    TIFR2: io8(0x17),
    tov2 = 0,
    ocf2a = 1,
    ocf2b = 2
);

crate::INIT_REG!(
    TIMSK0: mem8(0x6E),
    toie0 = 0,
    ocie0a = 1,
    ocie0b = 2
);

crate::INIT_REG!(
    TIMSK1: mem8(0x6F),
    toie1 = 0,
    ocie1a = 1,
    ocie1b = 2
);

crate::INIT_REG!(
    TIMSK2: mem8(0x70),
    toie2 = 0,
    ocie2a = 1,
    ocie2b = 2
);

crate::INIT_REG!(
    OCR0A: io8(0x27),
    ocr0a_0 = 0,
    ocr0a_1 = 1,
    ocr0a_2 = 2,
    ocr0a_3 = 3,
    ocr0a_4 = 4,
    ocr0a_5 = 5,
    ocr0a_6 = 6,
    ocr0a_7 = 7
);

crate::INIT_REG!(
    OCR0B: io8(0x28),
    ocr0b_0 = 0,
    ocr0b_1 = 1,
    ocr0b_2 = 2,
    ocr0b_3 = 3,
    ocr0b_4 = 4,
    ocr0b_5 = 5,
    ocr0b_6 = 6,
    ocr0b_7 = 7
);

crate::INIT_REG!(OCR1A: mem16(0x88));

crate::INIT_REG!(
    OCR1AL: mem8(0x88),
    ocr1al0 = 0,
    ocr1al1 = 1,
    ocr1al2 = 2,
    ocr1al3 = 3,
    ocr1al4 = 4,
    ocr1al5 = 5,
    ocr1al6 = 6,
    ocr1al7 = 7
);

crate::INIT_REG!(
    OCR1AH: mem8(0x89),
    ocr1ah0 = 0,
    ocr1ah1 = 1,
    ocr1ah2 = 2,
    ocr1ah3 = 3,
    ocr1ah4 = 4,
    ocr1ah5 = 5,
    ocr1ah6 = 6,
    ocr1ah7 = 7
);

crate::INIT_REG!(OCR1B: mem16(0x8A));

crate::INIT_REG!(
    OCR1BL: mem8(0x8A),
    ocr1bl0 = 0,
    ocr1bl1 = 1,
    ocr1bl2 = 2,
    ocr1bl3 = 3,
    ocr1bl4 = 4,
    ocr1bl5 = 5,
    ocr1bl6 = 6,
    ocr1bl7 = 7
);

crate::INIT_REG!(
    OCR1BH: mem8(0x8B),
    ocr1bh0 = 0,
    ocr1bh1 = 1,
    ocr1bh2 = 2,
    ocr1bh3 = 3,
    ocr1bh4 = 4,
    ocr1bh5 = 5,
    ocr1bh6 = 6,
    ocr1bh7 = 7
);

crate::INIT_REG!(
    OCR2A: mem8(0xB3),
    ocr2a_0 = 0,
    ocr2a_1 = 1,
    ocr2a_2 = 2,
    ocr2a_3 = 3,
    ocr2a_4 = 4,
    ocr2a_5 = 5,
    ocr2a_6 = 6,
    ocr2a_7 = 7
);

crate::INIT_REG!(
    OCR2B: mem8(0xB4),
    ocr2b_0 = 0,
    ocr2b_1 = 1,
    ocr2b_2 = 2,
    ocr2b_3 = 3,
    ocr2b_4 = 4,
    ocr2b_5 = 5,
    ocr2b_6 = 6,
    ocr2b_7 = 7
);
