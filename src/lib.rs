#![no_std]
#![feature(asm_experimental_arch)]
#![feature(abi_avr_interrupt)]
#![allow(static_mut_refs)]
#![allow(clippy::missing_safety_doc)] // TODO: Remove this

pub mod hardware_serial;
pub mod panic;
pub mod pins;
pub mod time;
pub mod volatile_cell;

pub use uno_hal_macro::entry;
pub use uno_hal_registers::init;
