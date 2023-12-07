use style4rs_util::{as_class_name, css_with_class_names};

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use quote::*;
use syn::Error;

#[proc_macro]
pub fn style(tokens: TokenStream) -> TokenStream {
    // TODO: try to use `syn::parse_macro_input(style_str(tokens))`...
    //
    let tokens: TokenStream2 = tokens.into();

    match css_with_class_names(&tokens) {
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

#[proc_macro]
pub fn style_str(tokens: TokenStream) -> TokenStream {
    let tokens: TokenStream2 = tokens.into();

    match css_with_class_names(&tokens) {
        Ok(css) => {
            let class_name = as_class_name(&tokens);
            quote! { (#class_name, #css) }.into()
        },
        Err(err) => {
            let err = format!("CSS error: {}", err);
            Error::new_spanned(tokens, err).to_compile_error().into()
        },
    }
}
