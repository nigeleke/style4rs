use style4rs_util::{as_class_name, byte_range};

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use quote::*;
use rsass::{compile_scss};
use syn::Error;

#[proc_macro]
pub fn style(tokens: TokenStream) -> TokenStream {
    let tokens: TokenStream2 = tokens.into();

    let mut css = String::new();
    let mut prev_span_end = byte_range(&tokens.clone().into_iter().next().unwrap().span()).end;

    for token in tokens.clone().into_iter() {
        let span = token.span();
        let text = span.source_text().unwrap();
        let this_range = byte_range(&span);

        if prev_span_end < this_range.start { css += " "; }
        css += &text;
        prev_span_end = this_range.end;
    }

    match compile_scss(css.as_bytes(), Default::default()) {
        Ok(_) => {
            let class_name = as_class_name(&tokens);
            quote! { #class_name }.into()
        },
        Err(err) => {
            let err = format!("CSS error: {}", err);
            Error::new_spanned(tokens, err).to_compile_error().into()
        },
    }

}
