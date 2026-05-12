use core::marker::PhantomData;

const __SFR_OFFSET: u8 = 0x20;

pub trait RO<T> {
    fn read(&self) -> T;
}

pub trait RW<T>: RO<T> {
    unsafe fn write(&mut self, value: T);
    unsafe fn update<F: FnOnce(T) -> T>(&mut self, f: F);
}

pub struct RegRO<T> {
    addr: u8,
    _marker: PhantomData<T>,
}
pub struct RegRW<T> {
    addr: u8,
    _marker: PhantomData<T>,
}

pub struct RegROSplit {
    pub l: RegRO<u8>,
    pub h: RegRO<u8>,
}

pub struct RegRWSplit {
    pub l: RegRW<u8>,
    pub h: RegRW<u8>,
}

impl<T> RegRW<T> {
    pub const fn as_ro(&self) -> RegRO<T> {
        RegRO::<T> {
            addr: self.addr,
            _marker: PhantomData,
        }
    }
}

impl RegRO<u8> {
    pub const fn new_io8<const ADDR: u8>() -> Self {
        Self {
            addr: ADDR + __SFR_OFFSET,
            _marker: PhantomData,
        }
    }

    pub const fn new_mem8<const ADDR: u8>() -> Self {
        Self {
            addr: ADDR,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn is_set(&self, bit: u8) -> bool {
        if bit < 8 {
            (self.read() & (1 << bit)) != 0
        } else {
            false
        }
    }

    #[inline]
    pub fn is_clear(&self, bit: u8) -> bool {
        !self.is_set(bit)
    }
}

impl RegRW<u8> {
    pub const fn new_io8<const ADDR: u8>() -> Self {
        Self {
            addr: ADDR + __SFR_OFFSET,
            _marker: PhantomData,
        }
    }

    pub const fn new_mem8<const ADDR: u8>() -> Self {
        Self {
            addr: ADDR,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn is_set(&self, bit: u8) -> bool {
        if bit < 8 {
            (self.read() & (1 << bit)) != 0
        } else {
            false
        }
    }

    #[inline]
    pub fn is_clear(&self, bit: u8) -> bool {
        !self.is_set(bit)
    }

    #[inline]
    pub unsafe fn set(&mut self, bit: u8) {
        if bit < 8 {
            self.update(|x| x | (1 << bit));
        }
    }

    #[inline]
    pub unsafe fn clear(&mut self, bit: u8) {
        if bit < 8 {
            self.update(|x| x & !(1 << bit));
        }
    }
}

impl RegRO<u16> {
    pub const fn new_io16<const ADDR: u8>() -> Self {
        Self {
            addr: ADDR + __SFR_OFFSET,
            _marker: PhantomData,
        }
    }

    pub const fn new_mem16<const ADDR: u8>() -> Self {
        Self {
            addr: ADDR,
            _marker: PhantomData,
        }
    }

    pub const fn split(&self) -> RegROSplit {
        RegROSplit {
            l: RegRO {
                addr: self.addr,
                _marker: PhantomData,
            },
            h: RegRO {
                addr: self.addr + 0x01,
                _marker: PhantomData,
            },
        }
    }
}

impl RegRW<u16> {
    pub const fn new_io16<const ADDR: u8>() -> Self {
        Self {
            addr: ADDR + __SFR_OFFSET,
            _marker: PhantomData,
        }
    }

    pub const fn new_mem16<const ADDR: u8>() -> Self {
        Self {
            addr: ADDR,
            _marker: PhantomData,
        }
    }

    pub const fn split(&self) -> RegRWSplit {
        RegRWSplit {
            l: RegRW {
                addr: self.addr,
                _marker: PhantomData,
            },
            h: RegRW {
                addr: self.addr + 0x01,
                _marker: PhantomData,
            },
        }
    }
}

impl<T> RO<T> for RegRO<T> {
    #[inline]
    fn read(&self) -> T {
        unsafe { (self.addr as *const T).read_volatile() }
    }
}

impl<T> RO<T> for RegRW<T> {
    #[inline]
    fn read(&self) -> T {
        unsafe { (self.addr as *const T).read_volatile() }
    }
}

impl<T> RW<T> for RegRW<T> {
    #[inline]
    unsafe fn write(&mut self, value: T) {
        (self.addr as *mut T).write_volatile(value);
    }

    #[inline]
    unsafe fn update<F: FnOnce(T) -> T>(&mut self, f: F) {
        self.write(f(self.read()));
    }
}
