use crate::{
    init_register,
    register::{RegRO, RegRW},
};

init_register! {
    #[doc = "USART Baud Rate Registers"]
    Ubbr0: RegRW<u16> = new_mem16(0xC4);
}
