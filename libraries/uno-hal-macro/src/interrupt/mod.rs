mod vectors;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, ItemFn, ReturnType, Type, Visibility, parse::Error, parse_macro_input, spanned::Spanned,
};

pub(crate) fn interrupt_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as ItemFn);

    if let Err(x) = check_signature(&func) {
        return x.to_compile_error().into();
    }

    let chip = match get_chip_name(args) {
        Ok(x) => x,
        Err(e) => return e.into_compile_error().into(),
    };

    let intr_name = func.sig.ident.clone().to_string();

    let Some(vector) = vectors::get_vector_id(&chip, &intr_name) else {
        return Error::new(
            proc_macro2::Span::call_site(),
            format!("Unknown chip `{chip}` or interrupt `{intr_name}`"),
        )
        .into_compile_error()
        .into();
    };

    let vector = Ident::new(
        &format!("__vector_{vector}"),
        proc_macro2::Span::call_site(),
    )
    .to_string();

    func.sig.ident = Ident::new(
        &format!("__interrupt_{}", func.sig.ident),
        proc_macro2::Span::call_site(),
    );

    let ident = func.sig.ident.clone();

    let caller = Ident::new(&format!("{ident}_caller"), proc_macro2::Span::call_site());

    quote!(
        #[doc(hidden)]
        #[unsafe(export_name = #vector)]
        unsafe extern "avr-interrupt" fn #caller() {
            #ident()
        }

        #[doc(hidden)]
        #[inline(always)]
        #func
    )
    .into()
}

fn get_chip_name(args: TokenStream) -> Result<String, Error> {
    let args: Vec<_> = args.into_iter().collect();

    let Some(chip) = args.first() else {
        return Err(Error::new(
            proc_macro2::Span::call_site(),
            "Interrupt chip must be specified: `#[interrupt(chip)]`",
        ));
    };

    let proc_macro::TokenTree::Ident(chip) = chip else {
        return Err(Error::new(
            proc_macro2::Span::call_site(),
            "Interrupt chip must be ident: `#[interrupt(chip)]`",
        ));
    };

    Ok(chip.to_string())
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
            ReturnType::Type(_, ref ty) => match **ty {
                Type::Tuple(ref tuple) => tuple.elems.is_empty(), // Return type is `()`
                Type::Never(..) => true, // Return type is `!`
                _ => false,
            },
            _ => true,
        };

    if !signature {
        Err(Error::new(
            func.span(),
            "Interrupts must have signature: `[unsafe] fn() [-> !]`",
        ))
    } else {
        Ok(())
    }
}
