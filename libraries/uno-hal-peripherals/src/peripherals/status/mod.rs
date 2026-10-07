mod registers;

use core::{
    arch::asm,
    cell::{Ref, RefCell, RefMut, UnsafeCell},
    marker::PhantomData,
};
pub use registers::*;

use crate::register::{RegRW, RO, RW};

#[inline]
pub unsafe fn enable_interrupts() {
    asm!("sei");
}

#[inline]
pub unsafe fn disable_interrupts() {
    asm!("cli");
}

pub struct Status;

impl Status {
    pub const unsafe fn reg() -> RegRW<u8> {
        Sreg::REG
    }

    pub fn interrupts() -> bool {
        Sreg::new().is_set_bit(SregBits::I)
    }
}

#[derive(Clone, Copy)]
pub struct CriticalSection<'cs> {
    _private: PhantomData<&'cs ()>,
    _not_send_sync: PhantomData<*mut ()>,
}

impl<'cs> CriticalSection<'cs> {
    #[inline(always)]
    pub unsafe fn new() -> Self {
        Self {
            _private: PhantomData,
            _not_send_sync: PhantomData,
        }
    }
}

#[inline]
pub fn atomic_block<R>(f: impl FnOnce(CriticalSection) -> R) -> R {
    struct Guard {
        state: u8,
    }

    impl Drop for Guard {
        #[inline(always)]
        fn drop(&mut self) {
            unsafe { Sreg::new().reg_mut().write(self.state) }
        }
    }

    let state = Sreg::REG.read();
    unsafe { disable_interrupts() };

    let _guard = Guard { state };

    unsafe { f(CriticalSection::new()) }
}

pub struct Mutex<T> {
    inner: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }

    #[inline]
    pub fn borrow<'cs>(&'cs self, _cs: CriticalSection<'cs>) -> &'cs T {
        unsafe { &*self.inner.get() }
    }
}

impl<T> Mutex<RefCell<T>> {
    #[inline]
    #[track_caller]
    pub fn replace<'cs>(&'cs self, cs: CriticalSection<'cs>, t: T) -> T {
        self.borrow(cs).replace(t)
    }

    #[inline]
    #[track_caller]
    pub fn replace_with<'cs, F>(&'cs self, cs: CriticalSection<'cs>, f: F) -> T
    where
        F: FnOnce(&mut T) -> T,
    {
        self.borrow(cs).replace_with(f)
    }

    #[inline]
    #[track_caller]
    pub fn borrow_ref<'cs>(&'cs self, cs: CriticalSection<'cs>) -> Ref<'cs, T> {
        self.borrow(cs).borrow()
    }

    #[inline]
    #[track_caller]
    pub fn borrow_ref_mut<'cs>(&'cs self, cs: CriticalSection<'cs>) -> RefMut<'cs, T> {
        self.borrow(cs).borrow_mut()
    }
}

impl<T: Default> Mutex<RefCell<T>> {
    #[inline]
    #[track_caller]
    pub fn take<'cs>(&'cs self, cs: CriticalSection<'cs>) -> T {
        self.borrow(cs).take()
    }
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}
