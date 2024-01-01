//! # Builder
//!
//! The `Builder` merges all `style!` and `style_sheet!` macro styling into a single target file, with
//! the component's `class_name` appended appropriately.
//!

pub mod builder;
mod error;
mod file_visitor;
mod result;
