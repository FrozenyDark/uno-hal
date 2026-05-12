#![no_std]
#![feature(asm_experimental_arch)]
#![allow(clippy::missing_safety_doc)] // TODO: Remove this

mod peripherals;

pub use peripherals::*;
pub const F_CPU: u32 = 16_000_000;
