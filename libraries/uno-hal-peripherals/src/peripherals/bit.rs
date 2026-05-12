use crate::addr::{RO, RW};

pub struct Bit<R: RO<u8>, const BIT: u8>(R);

impl<R: RO<u8>, const BIT: u8> Bit<R, BIT> {
    pub const fn new(reg: R) -> Self {
        Self(reg)
    }

    pub const fn bit(&self) -> u8 {
        BIT
    }

    #[inline]
    pub fn is_set(&self) -> bool {
        (self.0.read() & (1 << BIT)) != 0
    }

    #[inline]
    pub fn is_clear(&self) -> bool {
        !self.is_set()
    }
}

impl<R: RW<u8>, const BIT: u8> Bit<R, BIT> {
    #[inline]
    pub unsafe fn set(&mut self) {
        self.0.update(|x| x | (1 << BIT));
    }

    #[inline]
    pub unsafe fn clear(&mut self) {
        self.0.update(|x| x & !(1 << BIT));
    }

    #[inline]
    pub unsafe fn set_val(&mut self, val: bool) {
        match val {
            true => self.set(),
            false => self.clear(),
        }
    }

    #[inline]
    pub unsafe fn set_mask(&mut self, mask: u8) {
        self.set_val((mask & (1 << BIT)) > 0);
    }
}
