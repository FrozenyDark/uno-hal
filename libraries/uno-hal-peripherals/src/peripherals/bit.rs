use crate::addr::{RO8, RW8};

pub struct Bit<R: RO8, const BIT: u8>(R);

impl<R: RO8, const BIT: u8> Bit<R, BIT> {
    pub const fn new(reg: R) -> Self {
        Self(reg)
    }

    pub const fn bit(&self) -> u8 {
        BIT
    }

    pub const fn mask(&self) -> u8 {
        1 << BIT
    }

    #[inline]
    pub fn is_set(&self) -> bool {
        self.0.is_set(BIT)
    }

    #[inline]
    pub fn is_clear(&self) -> bool {
        !self.is_set()
    }
}

impl<R: RW8, const BIT: u8> Bit<R, BIT> {
    #[inline]
    pub unsafe fn set(&mut self) {
        self.0.set(BIT);
    }

    #[inline]
    pub unsafe fn clear(&mut self) {
        self.0.clear(BIT);
    }

    #[inline]
    pub unsafe fn set_val(&mut self, val: bool) {
        match val {
            true => self.set(),
            false => self.clear(),
        }
    }
}
