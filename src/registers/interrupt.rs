use crate::registers::*;

crate::INIT_REG!(
    SREG: io8(0x3F),
    sreg_c = 0,
    sreg_z = 1,
    sreg_n = 2,
    sreg_v = 3,
    sreg_s = 4,
    sreg_h = 5,
    sreg_t = 6,
    sreg_i = 7
);
