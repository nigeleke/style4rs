use style4rs_util::token_stream_to_class_name_and_css;

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use quote::*;
use syn::Error;

#[proc_macro]
pub fn style(tokens: TokenStream) -> TokenStream {
    let tokens: TokenStream2 = tokens.into();
    match token_stream_to_class_name_and_css(&tokens) {
        Ok((class_name, _)) => {
            quote! { #class_name }.into()
        },
        Err(err) => {
            let err = format!("CSS error: {}", err);
            Error::new_spanned(tokens, err).to_compile_error().into()
        },
    }
}

#[proc_macro]
pub fn style_str(tokens: TokenStream) -> TokenStream {
    let tokens: TokenStream2 = tokens.into();
    match token_stream_to_class_name_and_css(&tokens) {
        Ok((class_name, css)) => {
            quote! { (#class_name, #css) }.into()
        },
        Err(err) => {
            let err = format!("CSS error: {}", err);
            Error::new_spanned(tokens, err).to_compile_error().into()
        },
    }
}
