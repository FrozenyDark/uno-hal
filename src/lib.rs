#![no_std]
#![feature(asm_experimental_arch)]
#![feature(abi_avr_interrupt)]
#![allow(static_mut_refs)]
#![allow(clippy::missing_safety_doc)] // TODO: Remove this

mod delay;
pub mod panic;
pub mod peripherals;
pub mod volatile_cell;

pub use delay::*;
pub use uno_hal_macro::{entry, interrupt};
pub use uno_hal_peripherals::Peripherals;
