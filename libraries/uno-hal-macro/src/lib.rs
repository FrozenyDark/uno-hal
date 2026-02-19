mod entry;
mod interrupt;

extern crate proc_macro;

use proc_macro::TokenStream;

/// Macro for the entry function. Entry function must have signature `[unsafe] fn() -> !`.
///
/// # Usage
///
/// ```rust
/// #[uno_hal::entry]
/// fn main() -> ! {
///     /* Setup code */
///     loop {
///         /* Loop code */
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn entry(args: TokenStream, item: TokenStream) -> TokenStream {
    entry::entry_impl(args, item)
}

/// Macro for the interrupt handler function.
/// Takes chip name as an ident argument.
/// Handler's name must match interrupt's name.
/// Handler must have signature `[unsafe] fn() [-> !]`.
///
/// # Usage
///
/// Example of timer overflow handler for Arduino Uno with chip ATMEGA328P:
///
/// ```rust
/// #[uno_hal::interrupt(atmega328p)]
/// fn TIMER0_OVF() {
///     /* Handler code */
/// }
/// ```
#[proc_macro_attribute]
pub fn interrupt(args: TokenStream, item: TokenStream) -> TokenStream {
    interrupt::interrupt_impl(args, item)
}
