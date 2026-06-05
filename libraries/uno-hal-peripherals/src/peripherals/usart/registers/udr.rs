use crate::{
    init_register,
    register::{RegRO, RegRW},
};

init_register! {
    #[doc = "USART I/O Data Register"]
    Udr0: RegRW<u8> = new_mem8(0xC6);
}
