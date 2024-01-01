//! style4s-macros provides the macros.

use style4rs_util::{
    file_path_tokens_to_class_name_and_css,
    tokens_to_class_name_and_css
};

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use quote::*;
use syn::Error;

/// style! will return the unique class name for the css passed into it.
#[proc_macro]
pub fn style(tokens: TokenStream) -> TokenStream {
    let tokens: TokenStream2 = tokens.into();
    match tokens_to_class_name_and_css(&tokens) {
        Ok((class_name, _)) => {
            quote! { #class_name }.into()
        },
        Err(err) => {
            let err = format!("CSS error: {}", err);
            Error::new_spanned(tokens, err).to_compile_error().into()
        },
    }
}

/// style_str! will return the unique class name for the css passed into it,
/// together with the minimised css styling.
#[proc_macro]
pub fn style_str(tokens: TokenStream) -> TokenStream {
    let tokens: TokenStream2 = tokens.into();
    match tokens_to_class_name_and_css(&tokens) {
        Ok((class_name, css)) => {
            quote! { (#class_name, #css) }.into()
        },
        Err(err) => {
            let err = format!("CSS error: {}", err);
            Error::new_spanned(tokens, err).to_compile_error().into()
        },
    }
}

/// style_sheet! will return the unique class name for the css contained in
/// the given file.
#[proc_macro]
pub fn style_sheet(tokens: TokenStream) -> TokenStream {
    let tokens: TokenStream2 = tokens.into();
    match file_path_tokens_to_class_name_and_css(&tokens) {
        Ok((class_name, _)) => {
            quote! { #class_name }.into()
        },
        Err(err) => {
            let err = format!("Error: {}", err);
            Error::new_spanned(tokens, err).to_compile_error().into()
        },
    }
}

/// style_sheet_str! will return the unique class name for the css contained in
/// the given file, together with the minimised css.
#[proc_macro]
pub fn style_sheet_str(tokens: TokenStream) -> TokenStream {
    let tokens: TokenStream2 = tokens.into();
    match file_path_tokens_to_class_name_and_css(&tokens) {
        Ok((class_name, css)) => {
            quote! { (#class_name, #css) }.into()
        },
        Err(err) => {
            let err = format!("Error: {}", err);
            Error::new_spanned(tokens, err).to_compile_error().into()
        },
    }
}
