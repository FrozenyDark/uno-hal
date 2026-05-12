mod registers;

use core::arch::asm;
pub use registers::*;

#[inline]
pub unsafe fn enable_interrupts() {
    asm!("sei");
}

#[inline]
pub unsafe fn disable_interrupts() {
    asm!("cli");
}

#[macro_export]
macro_rules! atomic_block {
    ($($f:tt)*) => {{
        use $crate::{addr::{RO, RW}, status::{disable_interrupts, Sreg}};

        let __old_sreg = Sreg::REG.read();
        unsafe { disable_interrupts() }
        let __res = { $($f)* };
        unsafe { Sreg::REG.write(__old_sreg) }

        __res
    }};
}
