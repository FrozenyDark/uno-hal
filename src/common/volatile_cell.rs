use core::cell::UnsafeCell;

#[derive(Debug)]
pub struct VolatileCell<T> {
    value: UnsafeCell<T>,
}

impl<T> VolatileCell<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn read(&self) -> T {
        unsafe { self.value.get().read_volatile() }
    }

    #[inline]
    pub fn write(&mut self, value: T) {
        unsafe { self.value.get().write_volatile(value) }
    }

    #[inline]
    pub fn update<F>(&mut self, f: F)
    where
        F: FnOnce(T) -> T,
    {
        self.write(f(self.read()));
    }
}
