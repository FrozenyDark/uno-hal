mod entry;
mod interrupt;

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn entry(args: TokenStream, item: TokenStream) -> TokenStream {
    entry::entry_impl(args, item)
}

#[proc_macro_attribute]
pub fn interrupt(args: TokenStream, item: TokenStream) -> TokenStream {
    interrupt::interrupt_impl(args, item)
}
