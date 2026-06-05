use crate::{
    init_register,
    register::{RegRO, RegRW},
};

init_register! {
    #[doc = "Timer/Counter 0 Register"]
    Tcnt0: RegRW<u8> = new_io8(0x26);
}

init_register! {
    #[doc = "Timer/Counter 1 Register"]
    Tcnt1: RegRW<u16> = new_mem16(0x84);
}

init_register! {
    #[doc = "Timer/Counter 2 Register"]
    Tcnt2: RegRW<u8> = new_mem8(0xB2);
}
