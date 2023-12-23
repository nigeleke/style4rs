use lightningcss::{
    selector::{Combinator, Component, PseudoClass, Selector},
    stylesheet::{StyleSheet, ParserOptions, PrinterOptions},
    visitor::{Visit, Visitor, VisitTypes},
    visit_types,
};
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Span, TokenStream, TokenTree};
use regex::Regex;

use std::collections::hash_map::DefaultHasher;
use std::hash::*;
use std::ops::Range;

/// Recover the original source css embedded in the tokenstream.
/// Note: this can only be called from a macro invocation. The source
///       needs to be recovered directly from the source file otherwise.
///
pub fn source_from(tokens: &TokenStream) -> String {
    let group = Group::new(Delimiter::None, tokens.clone());
    let source = group.span().source_text().unwrap();

    let tokens = Vec::from_iter(tokens.clone().into_iter());

    let group_range = byte_range(&group.span());
    let (first_range, last_range) = 
        if tokens.len() > 0 {
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

/// Create a deterministic class name based on the TokenStream.
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
    let re = Regex::new(r"^(#\d+ )?bytes\((\d+)\.\.(\d+)\)$").unwrap();
    let captures = re.captures(span).unwrap();
    let start = captures[2].parse::<usize>().unwrap();
    let end = captures[3].parse::<usize>().unwrap();
    start..end
}

/// Compile css and return with deterministic class name inserted.
///
pub fn css_with_class_names(input: &String, class_name: &String) -> Result<String, String> {
    insert_class_name(&input, &class_name)
}

#[derive(Clone, Debug, Default)]
struct InserterState<'a> {
    class_name: String,
    insert_required: bool,
    previous_combinator: Option<Combinator>,
    current: Vec<Component<'a>>,
    updated: Vec<Vec<Component<'a>>>,
}

impl<'a> InserterState<'a> {
    fn new(class_name: &String) -> Self {
        Self { class_name: class_name.clone(), ..InserterState::default() }
    }

    fn set_for_next_selector(&mut self) {
        self.current = Vec::new();
        self.insert_required = self.previous_combinator != Some(Combinator::PseudoElement);
    }

    fn persist_selector(&mut self) {
        self.updated.push(self.current.clone());
    }

    fn push_previous_combinator(&mut self) {
        if let Some(c) = self.previous_combinator {
            self.push_component(&Component::Combinator(c.clone()));
            self.previous_combinator = None;
        }
    }

    fn push_combinator_and_persist_selector(&mut self, c: &Combinator) {
        self.previous_combinator = Some(c.clone());
        self.persist_selector();
    }

    fn push_component(&mut self, c: &Component<'a>) {
        self.current.push(c.clone());
    }

    fn insert_class_if_needed(&mut self) {
        if self.insert_required {
            let class = Component::Class(self.class_name.clone().into());
            self.push_component(&class);
            self.insert_required = false;
        }
    } 

    fn skip_insert_class(&mut self) {
        self.insert_required = false;
    }

    fn finalised_selector(&mut self) -> Selector<'a> {
        let updated_vec: Vec<Component> = self.updated
            .iter()
            .rev()
            .flat_map(|i| i.iter().cloned())
            .collect();
        self.updated.clear();
        self.set_for_next_selector();
        Selector::from(updated_vec.clone())
    }
}

struct CustomClassInserter<'a> {
    state: InserterState<'a>,
}

impl CustomClassInserter<'_> {
    fn new(class_name: &String) -> Self {
        Self { state: InserterState::new(class_name), }
    }
}

impl<'i> Visitor<'i> for CustomClassInserter<'i> {
    type Error = ();

    fn visit_types(&self) -> VisitTypes {
        visit_types!(SELECTORS)
    }

    fn visit_selector(&mut self, selector: &mut Selector<'i>) -> Result<(), Self::Error> {
        let mut iter = selector.iter();

        loop {
            self.state.set_for_next_selector();

            while let Some(s) = iter.next() {
                match s {
                    Component::ExplicitUniversalType => { /* * replaced by rcn class */ },
                    Component::Negation(_) |
                    Component::Nth(_) |
                    Component::PseudoElement(_) => {
                        self.state.insert_class_if_needed();
                        self.state.push_component(s);
                    },
                    Component::NonTSPseudoClass(pc) => {
                        match pc {
                            PseudoClass::CustomFunction { .. } => {
                                self.state.skip_insert_class();          
                                self.state.push_component(s);
                            },
                            _ => {
                                self.state.insert_class_if_needed();
                                self.state.push_component(s);
                            }
                        }
                    },
                    Component::Root => { 
                        self.state.skip_insert_class();
                        self.state.push_component(s);
                    },
                    _ => { self.state.push_component(s); },
                }
            }

            self.state.insert_class_if_needed();
            self.state.push_previous_combinator();

            if let Some(c) = iter.next_sequence() {
                self.state.push_combinator_and_persist_selector(&c);
            } else {
                self.state.persist_selector();
                break;
            };
        }

        let updated = self.state.finalised_selector(); 
        *selector = updated;

        Ok(())
    }
}

fn insert_class_name(css: &String, class_name: &String) -> Result<String, String> {
    let mut inserter = CustomClassInserter::new(class_name);
    let parser_options = ParserOptions::default();
    let mut stylesheet = StyleSheet::parse(css, parser_options).unwrap();
    let _ = stylesheet.visit(&mut inserter);
    let printer_options = PrinterOptions { minify: true, ..PrinterOptions::default() };
    let res = stylesheet.to_css(printer_options).unwrap();
    Ok(res.code)
}
