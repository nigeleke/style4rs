use style4rs_util::as_class_name;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::*;

#[proc_macro]
pub fn style(tokens: TokenStream) -> TokenStream {
    let tokens: TokenStream2 = tokens.into();
    let class_name = as_class_name(&tokens);
    quote! { #class_name }.into()
}
