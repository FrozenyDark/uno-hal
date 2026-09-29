use crate::{
    init_bits, init_register,
    register::{BitRO, BitRW, RegRO, RegRW},
};

init_bits! {
    EecrBits {
        #[doc = "EEPROM Read Enable"]
        EERE = 0,
        #[doc = "EEPROM Write Enable"]
        EEPE = 1,
        #[doc = "EEPROM Master Write Enable"]
        EEMPE = 2,
        #[doc = "EEPROM Ready Interrupt Enable"]
        EERIE = 3,
        #[doc = "EEPROM Programming Mode Bit"]
        EEPM0 = 4,
        #[doc = "EEPROM Programming Mode Bit"]
        EEOM1 = 5,
    }
}

init_register! {
    #[doc = "EEPROM Address Register"]
    Eear: RegRW<u16> = new_io16(0x21);
}

init_register! {
    #[doc = "EEPROM Data Register"]
    Eedr: RegRW<u8> = new_io8(0x20);
}

init_register! {
    #[doc = "EEPROM Control Register"]
    Eecr: RegRW<u8> = new_io8(0x1F) + EecrBits
}
