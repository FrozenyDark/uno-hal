pub mod registers;

use registers::*;

use crate::register::{RO, RW};

pub struct Eeprom {
    pub eear: Eear,
    pub eedr: Eedr,
    pub eecr: Eecr,
}

impl Eeprom {
    pub const MAX_SIZE: u16 = 1024;
    const MASK: u16 = 0x3FF;

    pub(crate) const fn new() -> Self {
        Self {
            eear: Eear::new(),
            eedr: Eedr::new(),
            eecr: Eecr::new(),
        }
    }

    #[inline]
    pub fn is_ready(&self) -> bool {
        self.eecr.is_clear_bit(EecrBits::EEPE)
    }

    #[inline]
    pub fn busy_wait(&self) {
        while !self.is_ready() {}
    }

    pub unsafe fn write(&mut self, address: u16, data: u8) -> Result<(), &'static str> {
        if address >= Self::MAX_SIZE {
            return Err("Out of bounds");
        }

        self.busy_wait();

        self.eear.reg_mut().write(address & Self::MASK);
        self.eedr.reg_mut().write(data);

        self.eecr.set_bit(EecrBits::EEMPE);
        self.eecr.set_bit(EecrBits::EEPE);

        Ok(())
    }

    pub unsafe fn read(&mut self, address: u16) -> Result<u8, &'static str> {
        if address >= Self::MAX_SIZE {
            return Err("Out of bounds");
        }

        self.busy_wait();

        self.eear.reg_mut().write(address & Self::MASK);

        self.eecr.set_bit(EecrBits::EERE);

        Ok(self.eedr.reg().read())
    }
}
