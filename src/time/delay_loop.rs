use core::arch::asm;

// Delays up to 768 / (CPU Speed) microseconds, where CPU speed is in MHz.
#[inline]
#[allow(unused_assignments)]
pub unsafe fn _delay_loop_1(mut c: u8) {
    asm!(
        "1:",
        "dec {x}",
        "brne 1b",
        x = inout(reg) c
    )
}

// Delays up to 262.1 / (CPU Speed) milliseconds, where CPU speed is in MHz.
#[inline]
#[allow(unused_assignments)]
pub unsafe fn _delay_loop_2(mut c: u16) {
    asm!(
        "1:",
        "sbiw {x}, 1", // 2 cycles
        "brne 1b", // 2 cycles
        x = inout(reg_iw) c
    )
}
