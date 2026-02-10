use crate::registers::{_SFR_IO16, _SFR_IO8, _SFR_MEM16, _SFR_MEM8};

pub struct RegisterCell<T> {
    cell: *mut T,
}

impl RegisterCell<u8> {
    pub(in super::super) const fn new_io8(addr: u8) -> Self {
        Self {
            cell: _SFR_IO8(addr),
        }
    }

    pub(in super::super) const fn new_mem8(addr: u8) -> Self {
        Self {
            cell: _SFR_MEM8(addr),
        }
    }

    #[inline]
    pub fn set(&self, bit: u8) {
        self.update(|x| x | (1 << bit));
    }

    #[inline]
    pub fn clear(&self, bit: u8) {
        self.update(|x| x & !(1 << bit));
    }

    #[inline]
    pub fn is_set(&self, bit: u8) -> bool {
        (self.read() & (1 << bit)) != 0
    }

    #[inline]
    pub fn is_clear(&self, bit: u8) -> bool {
        !self.is_set(bit)
    }
}

impl RegisterCell<u16> {
    pub(in super::super) const fn new_io16(addr: u16) -> Self {
        Self {
            cell: _SFR_IO16(addr),
        }
    }

    pub(in super::super) const fn new_mem16(addr: u16) -> Self {
        Self {
            cell: _SFR_MEM16(addr),
        }
    }
}

impl<T> RegisterCell<T> {
    #[inline]
    pub fn read(&self) -> T {
        unsafe { self.cell.read_volatile() }
    }

    #[inline]
    pub fn write(&self, value: T) {
        unsafe { self.cell.write_volatile(value) };
    }

    #[inline]
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(T) -> T,
    {
        self.write(f(self.read()));
    }
}
