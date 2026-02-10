use core::marker::PhantomData;

use super::Register;

pub struct RegisterBit<T, const BIT: u8>
where
    T: Register<u8>,
{
    _marker: PhantomData<T>,
}

impl<T, const BIT: u8> RegisterBit<T, BIT>
where
    T: Register<u8>,
{
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub const fn bit(&self) -> u8 {
        BIT
    }

    #[inline]
    pub fn set(&self) {
        T::REGISTER.set(BIT);
    }

    #[inline]
    pub fn clear(&self) {
        T::REGISTER.clear(BIT);
    }

    #[inline]
    pub fn is_set(&self) -> bool {
        T::REGISTER.is_set(BIT)
    }

    #[inline]
    pub fn is_clear(&self) -> bool {
        T::REGISTER.is_clear(BIT)
    }
}
