use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};
use regex::Regex;

use std::collections::hash_map::DefaultHasher;
use std::hash::*;
use std::ops::Range;

fn hash_delimiter(delimiter: Delimiter, hasher: &mut DefaultHasher) {
    hasher.write(format!("{:?}", delimiter).as_bytes());
} 

fn hash_tree(input: &TokenTree, hasher: &mut DefaultHasher) {
    match input {
        TokenTree::Group(t) => {
            hash_delimiter(t.delimiter(), hasher);
            hash_stream(&t.stream(), hasher);
        },
        TokenTree::Ident(t) => { t.hash(hasher); },
        TokenTree::Punct(t) => { hasher.write(t.to_string().as_bytes()); },
        TokenTree::Literal(t) => { hasher.write(t.to_string().as_bytes()); },
    };
}

fn hash_stream(input: &TokenStream, hasher: &mut DefaultHasher) {
    for token in input.clone().into_iter() {
        hash_tree(&token, hasher);
    }
}

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
