use core::arch::asm;

#[inline]
pub unsafe fn enable() {
    asm!("sei");
}

#[inline]
pub unsafe fn disable() {
    asm!("cli");
}

#[macro_export]
macro_rules! atomic_block {
    ($($f:stmt)*) => {{
        use $crate::{interrupt::disable, registers::{interrupt::SREG, Register}};

        let __old_sreg = SREG.read();
        unsafe { disable() };
        let __res = { $($f)* };
        SREG.write(__old_sreg);

        __res
    }};
}
