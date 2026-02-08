use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, ItemFn, ReturnType, Type, Visibility, parse::Error, parse_macro_input, spanned::Spanned,
};

pub(crate) fn entry_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as ItemFn);

    if let Err(x) = check_signature(&func) {
        return x.to_compile_error().into();
    }

    func.sig.ident = Ident::new(
        &format!("__entry_point_{}__", func.sig.ident),
        proc_macro2::Span::call_site(),
    );

    quote!(
        #[doc(hidden)]
        #[unsafe(export_name = "main")]
        unsafe extern "C" #func
    )
    .into()
}

fn check_signature(func: &ItemFn) -> Result<(), Error> {
    let signature = func.sig.constness.is_none() // Non constant
        && matches!(func.vis, Visibility::Inherited) // Private
        && func.sig.abi.is_none() // No `extern "C"`
        && func.sig.inputs.is_empty() // No arguments
        && func.sig.generics.params.is_empty() // No generics
        && func.sig.generics.where_clause.is_none() // No `where` clause
        && func.sig.variadic.is_none() // No variadics (`...` arguments)
        && match func.sig.output {
            ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_)), // Return type is `!`
            _ => false,
        };

    if !signature {
        Err(Error::new(
            func.span(),
            "Entry func must have signature: `[unsafe] fn() -> !`",
        ))
    } else {
        Ok(())
    }
}
