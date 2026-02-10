#![allow(dead_code)]
#![allow(non_snake_case)]

pub mod analog;
mod constants;
pub mod digital;
pub mod interrupt;
mod register;
pub mod timers;
pub mod usart;

pub use constants::*;
pub use register::*;

const __SFR_OFFSET: u8 = 0x20;

const fn _SFR_IO8(addr: u8) -> *mut u8 {
    (addr + __SFR_OFFSET) as *mut u8
}

const fn _SFR_IO16(addr: u16) -> *mut u16 {
    (addr + __SFR_OFFSET as u16) as *mut u16
}

const fn _SFR_MEM8(addr: u8) -> *mut u8 {
    addr as *mut u8
}

const fn _SFR_MEM16(addr: u16) -> *mut u16 {
    addr as *mut u16
}

#[macro_export]
macro_rules! IMPL_REG {
    ($name:tt: io8($addr:literal)) => {
        impl $crate::registers::Register<u8> for $name {
            const REGISTER: $crate::registers::RegisterCell<u8> = RegisterCell::new_io8($addr);
        }
    };

    ($name:tt: mem8($addr:literal)) => {
        impl $crate::registers::Register<u8> for $name {
            const REGISTER: $crate::registers::RegisterCell<u8> = RegisterCell::new_mem8($addr);
        }
    };

    ($name:tt: io16($addr:literal)) => {
        impl $crate::registers::Register<u16> for $name {
            const REGISTER: $crate::registers::RegisterCell<u16> = RegisterCell::new_io16($addr);
        }
    };

    ($name:tt: mem16($addr:literal)) => {
        impl $crate::registers::Register<u16> for $name {
            const REGISTER: $crate::registers::RegisterCell<u16> = RegisterCell::new_mem16($addr);
        }
    };
}

#[macro_export]
macro_rules! INIT_REG {
    ($name:tt: $type:tt($addr:literal)) => {
        #[allow(clippy::upper_case_acronyms)]
        pub struct $name {}

        $crate::IMPL_REG!($name: $type($addr));

        pub const $name: $name = $name {};
    };

    ($name:tt: $type:tt($addr:literal), $($bit_name:tt = $bit:literal),+) => {
        #[allow(clippy::upper_case_acronyms)]
        pub struct $name {
            $(
                pub $bit_name: RegisterBit<Self, $bit>
            ),+
        }

        $crate::IMPL_REG!($name: $type($addr));

        pub const $name: $name = $name {
            $(
                $bit_name: RegisterBit::new()
            ),+
        };
    };
}
