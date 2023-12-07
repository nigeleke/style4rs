// mod dummy_selector_impl;

// use dummy_selector_impl::tests::DummySelectorImpl;

use lightningcss::printer::PrinterOptions;
use lightningcss::selector::Selector;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use lightningcss::visitor::{Visit, Visitor, VisitTypes};
use lightningcss::visit_types;
use parcel_selectors::parser::Component;
use parcel_selectors::visitor::SelectorVisitor;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Span, TokenStream, TokenTree};
use regex::Regex;
use selectors::parser::SelectorImpl;

use std::collections::hash_map::DefaultHasher;
use std::hash::*;
use std::ops::Range;
use std::str;

/// Create a deterministic class name based on the TokenStream.
/// Note: It isn't possible to simply convert the stream to a string, and hash that because:
///       a) This is called from the style! invocation and by the Style4rsBuilder.
///       b) The style! invocation contains Span metadata, whereas the Style4rsBuilder does not.
///       c) The Span metadata impacts the hash.
///
pub fn as_class_name(input: &TokenStream) -> String {
    let mut hasher = DefaultHasher::new();
    hash_stream(input, &mut hasher);
    let hash = hasher.finish();
    format!("rcn-{:x}", hash)
}

fn hash_stream(input: &TokenStream, hasher: &mut DefaultHasher) {
    for token in input.clone().into_iter() {
        hash_tree(&token, hasher);
    }
}

#[inline]
fn hash_tree(input: &TokenTree, hasher: &mut DefaultHasher) {
    match input {
        TokenTree::Group(t) => { hash_group(t, hasher); },
        TokenTree::Ident(t) => { hash_ident(t, hasher); },
        TokenTree::Punct(t) => { hash_punct(t, hasher); },
        TokenTree::Literal(t) => { hash_literal(t, hasher); },
    };
}

#[inline]
fn hash_group(group: &Group, hasher: &mut DefaultHasher) {
    hash_delimiter(&group.delimiter(), hasher);
    hash_stream(&group.stream(), hasher);
}

#[inline]
fn hash_delimiter(delimiter: &Delimiter, hasher: &mut DefaultHasher) {
    hasher.write(format!("{:?}", delimiter).as_bytes());
} 

#[inline]
fn hash_ident(ident: &Ident, hasher: &mut DefaultHasher) {
    ident.hash(hasher);
}

#[inline]
fn hash_punct(punct: &Punct, hasher: &mut DefaultHasher) {
    hasher.write(punct.to_string().as_bytes());
}

#[inline]
fn hash_literal(literal: &Literal, hasher: &mut DefaultHasher) {
    hasher.write(literal.to_string().as_bytes());
}

/// Emualate the `proc_macro::Span::byte_range` method.
/// Rely on the debug format of `#n bytes(from..to))`.
/// TODO: Remove this when `byte_range` becomes available on the stable build...
///
pub fn byte_range(span: &Span) -> Range<usize> {
    let span = &format!("{:?}", span);
    let re = Regex::new(r"^#0 bytes\((\d+)\.\.(\d+)\)$").unwrap();
    let captures = re.captures(span).unwrap();
    let start = captures[1].parse::<usize>().unwrap();
    let end = captures[2].parse::<usize>().unwrap();
    start..end
}

/// Compile proc_macro2::TokenStream to css with deterministic
/// class name inserted.
///
pub fn css_with_class_names(input: &TokenStream) -> Result<String, String> {
    let class_name = as_class_name(input);
    let css = reconstruct_text(input);
    insert_class_name(&css, &class_name)
}

fn reconstruct_text(input: &TokenStream) -> String {
    let mut css = String::new();

    let mut iter = input.clone().into_iter();
    if let Some(token) = iter.next() {
        let mut prev_span_end = byte_range(&token.span()).end;
        css += &token.span().source_text().unwrap();
        
        for token in iter {
            let span = token.span();
            let text = span.source_text().unwrap();
            let this_range = byte_range(&span);
        
            if prev_span_end < this_range.start { css += " "; }
            css += &text;
            prev_span_end = this_range.end;
        }
    }
    
    css
}

fn insert_class_name(css: &String, class_name: &String) -> Result<String, String> {
    match StyleSheet::parse(css, ParserOptions::default()) {
        Ok(mut stylesheet) => {
            let css = insert_class_name_into_stylesheet(&mut stylesheet, class_name);
            Ok(css)
        },
        Err(err) => {
            println!("Err {:?}", err);
            Err(err.to_string())
        },
    }
}

struct ClassNameInserter {
    class_name: String,
}

impl ClassNameInserter {
    fn new(class_name: String) -> Self {
        Self { class_name: format!("{}{}", '.', class_name) }
    }

    fn append_class_name(&self, src: &[u8]) -> String {
        let mut src = str::from_utf8(src).unwrap().to_string();
        src += &self.class_name;
        src
    }
}

impl<'a> Visitor<'a> for ClassNameInserter {
    type Error = String;

    fn visit_types(&self) -> VisitTypes {
        visit_types!(SELECTORS)
    }

    fn visit_selector(&mut self, selector: &mut Selector<'a>) -> Result<(), Self::Error> {
        //let identifier: Ident = self.class_name.clone().into();
        println!(r#"
***** pre  visit_selector: {:?}
      is_universal {}
      len: {}
      has_combinator: {}
      is_slotted: {}
      is_part: {}
      parts: {:?}
      class_name: {:?}"#, 
            selector, 
            selector.is_universal(),
            selector.len(),
            selector.has_combinator(),
            selector.is_slotted(),
            selector.is_part(),
            selector.parts(),
            self.class_name);
        
        // let iter = &selector.iter();
        // for i in *iter {
        //     if let Component::Class(class) = i {
        //         println!("         class {:?} => {:?}", i, class);
        //         // let x = self.append_class_name(class.as_bytes());
        //         // println!("Appending {}", x);
        //         // println!("      src {:?}", class);
        //         // *class = x.into();
        //         // println!("      dst {:?}", class);
        //     } else 
        //     if let Component::Combinator(combinator) = i {
        //         println!("         combinator {:?} => {:?}", i, combinator);
        //     } else 
        //     if let Component::ID(id) = i {
        //         println!("         id {:?} => {:?}", i, id);
        //     } else 
        //     if let Component::LocalName(name) = i {
        //         println!("         local_name {:?} => {:?}", i, name);
        //     }
        // }

        // let x = *iter.next_sequence();

        // println!("***** post visit_selector: {:?} {:?}", x, selector);

        Ok(())
    }
}

impl<'a> SelectorVisitor<'a> for ClassNameInserter {
    type Impl = dyn SelectorImpl;
}

fn insert_class_name_into_stylesheet(stylesheet: &mut StyleSheet, class_name: &String) -> String {
    let printer_options = PrinterOptions { minify: true, ..PrinterOptions::default() };
    let mut inserter = ClassNameInserter::new(class_name.to_string());
    stylesheet.visit(&mut inserter).unwrap();
    stylesheet.to_css(printer_options).unwrap().code
}
