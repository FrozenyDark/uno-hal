mod register_bit;
mod register_cell;

pub use register_bit::*;
pub use register_cell::*;

pub trait Register<T> {
    const REGISTER: RegisterCell<T>;

    #[inline]
    fn read(&self) -> T {
        Self::REGISTER.read()
    }

    #[inline]
    fn write(&self, value: T) {
        Self::REGISTER.write(value);
    }

    #[inline]
    fn update<F>(&self, f: F)
    where
        F: FnOnce(T) -> T,
    {
        Self::REGISTER.update(f);
    }
}
