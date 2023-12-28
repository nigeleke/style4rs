use style4rs_util::{css_to_css_with_class_name, str_as_class_name, tokens_as_class_name};

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Span, TokenStream as TokenStream2};
use quote::*;
use regex::Regex;
use syn::Error;

use std::ops::Range;

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

/// Recover the original source css embedded in the tokenstream.
/// Note: this can only be called from a macro invocation. The source
///       needs to be recovered directly from the source file otherwise.
///
fn source_from(tokens: &TokenStream2) -> String {
    let group = Group::new(Delimiter::None, tokens.clone());
    let source = group.span().source_text().unwrap();

    let tokens = Vec::from_iter(tokens.clone());

    let group_range = byte_range(&group.span());
    let (first_range, last_range) = 
        if !tokens.is_empty() {
            let len = tokens.len();
            (byte_range(&tokens[0].span()), byte_range(&tokens[len-1].span()))
        } else {
            (group_range.clone(), group_range.clone())
        };

    let start_offset = first_range.start - group_range.start;
    let length = last_range.end - first_range.start;
    let end_offset = start_offset + length;

    source[start_offset..end_offset].to_string()
}

/// Emualate the `proc_macro::Span::byte_range` method.
/// Rely on the debug format of `#n bytes(from..to))`.
/// TODO: Remove this when `byte_range` becomes available on the stable build...
///
fn byte_range(span: &Span) -> Range<usize> {
    let span = &format!("{:?}", span);
    let re = Regex::new(r"^(#\d+ )?bytes\((\d+)\.\.(\d+)\)$").unwrap();
    let captures = re.captures(span).unwrap();
    let start = captures[2].parse::<usize>().unwrap();
    let end = captures[3].parse::<usize>().unwrap();
    start..end
}

/// Return css with deterministic class name inserted, from a `proc_macro::TokenStream` source.
///
fn tokens_to_class_name_and_css(tokens: &TokenStream2) -> Result<(String, String), String> {
    let source = source_from(tokens);
    let class_name = tokens_as_class_name(tokens);
    match css_to_css_with_class_name(&source, &class_name) {
        Ok(css) => { Ok((class_name, css)) },
        Err(err) => { Err(err) },
    }
}

/// Return css with deterministic class name inserted, from a file referenced in the `proc_macro::TokenStream` source.
///
fn file_path_tokens_to_class_name_and_css(tokens: &TokenStream2) -> Result<(String, String), String> {
    let file_path = tokens.to_string();
    let file_path = file_path.trim_matches('"');
    let source = std::fs::read_to_string(file_path).expect("Expected to read file");
    let class_name = str_as_class_name(&source);
    match css_to_css_with_class_name(&source, &class_name) {
        Ok(css) => { Ok((class_name, css)) },
        Err(err) => { Err(err) },
    }
}
