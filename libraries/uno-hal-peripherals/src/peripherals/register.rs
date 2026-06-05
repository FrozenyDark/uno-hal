use core::marker::PhantomData;

const __SFR_OFFSET: u8 = 0x20;

pub trait RO<T> {
    fn read(&self) -> T;
}

pub trait RW<T>: RO<T> {
    unsafe fn write(&mut self, value: T);
    unsafe fn update<F: FnOnce(T) -> T>(&mut self, f: F);
}

pub trait BitRO: RO<u8> {
    #[inline]
    fn is_set(&self, mask: u8) -> bool {
        (self.read() & mask) == mask
    }

    #[inline]
    fn is_clear(&self, mask: u8) -> bool {
        !self.is_set(mask)
    }

    #[inline]
    fn is_set_bit(&self, bit: u8) -> bool {
        self.is_set(1 << bit)
    }

    #[inline]
    fn is_clear_bit(&self, bit: u8) -> bool {
        self.is_clear(1 << bit)
    }
}

pub trait BitRW: BitRO + RW<u8> {
    #[inline]
    unsafe fn set(&mut self, mask: u8) {
        self.update(|x| x | mask);
    }

    #[inline]
    unsafe fn clear(&mut self, mask: u8) {
        self.update(|x| x & !mask);
    }

    #[inline]
    unsafe fn set_bit(&mut self, bit: u8) {
        self.set(1 << bit)
    }

    #[inline]
    unsafe fn clear_bit(&mut self, bit: u8) {
        self.clear(1 << bit)
    }
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

impl BitRO for RegRO<u8> {}

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

impl BitRO for RegRW<u8> {}
impl BitRW for RegRW<u8> {}
