use crate::{
    init_register,
    register::{RegRO, RegRW},
};

init_register! {
    #[doc = "Output Compare Register 0 A"]
    Ocr0A: RegRW<u8> = new_io8(0x27);
}

init_register! {
    #[doc = "Output Compare Register 0 B"]
    Ocr0B: RegRW<u8> = new_io8(0x28);
}

init_register! {
    #[doc = "Output Compare Register 1 A"]
    Ocr1A: RegRW<u16> = new_mem16(0x88);
}

init_register! {
    #[doc = "Output Compare Register 1 B"]
    Ocr1B: RegRW<u16> = new_mem16(0x8A);
}

init_register! {
    #[doc = "Output Compare Register 2 A"]
    Ocr2A: RegRW<u8> = new_mem8(0xB3);
}

init_register! {
    #[doc = "Output Compare Register 2 B"]
    Ocr2B: RegRW<u8> = new_mem8(0xB4);
}
