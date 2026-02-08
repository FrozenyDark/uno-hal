mod entry;

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn entry(args: TokenStream, item: TokenStream) -> TokenStream {
    entry::entry_impl(args, item)
}
