//! # Style4rs
//!
//! Style4rs allows [Leptos](https://leptos.dev/) and [Yew](https://yew.rs/) components to encapsulate their own
//! css without encroaching on css defined for other components.
//!
//! It comprises two main areas of functionality; the [macros](../style4rs_macros/#) to enable unique component
//! classes to be defined, and the [builder](../style4rs_builder/#) which merges the components' unique css into
//! a single file.
//!
//! ## Macros
//!
//! ### style!
//! `style!` will return the unique class name for the css passed into it.
//! The build process will place the minimised css in a target file.
//! ```ignore // style4rs will not process in-comment code.
//! use style4rs::style;
//! let class_name = style!{
//!   color: white;
//!   background-color: red;
//! };
//! ```
//! This example is _untested_ as `style4rs` will not process in-comment code.
//!
//! ### style_str!
//! `style_str!` will return the unique class name for the css passed into it,
//! together with the minimised css styling.
//! The build process will NOT process this macro.
//! ```ignore // style4rs will not process in-comment code.
//! use style4rs::style_str;
//! let (class_name, style) = style_str!{
//!   color: white;
//!   background-color: red;
//! };
//! ```
//! This example is _untested_ as `style4rs` will not process in-comment code.
//!
//! ### style_sheet!
//! `style_sheet!` will return the unique class name for the css contained in
//! the given file.
//! The build process will place the minimised css in a target file.
//! ```
//! use style4rs::style_sheet;
//! let class_name = style_sheet!("examples/resources/style_sheet.css");
//! ```
//!
//! ### style_sheet_str!
//! `style_sheet_str!` will return the unique class name for the css contained in
//! the given file, together with the minimised css.
//! The build process will NOT process this macro.
//! ```
//! use style4rs::style_sheet_str;
//! let (class_name, style) = style_sheet_str!("examples/resources/style_sheet_str.css");
//! ```
//!
//! ## Builder
//!
//! The `Builder` merges all `style!` and `style_sheet!` macro styling into a single target file, with
//! the component's `class_name` appended appropriately.
//!
//! The default method of calling Builder is:
//! ```
//! use style4rs::Builder;
//! Builder::new().build().ok();
//! ```
//!
//! The default input path is `$env:CARGO_MANIFEST_DIR/src`. This can be overridden with `using_in_folder(...)`.
//! The default output file is `$env:OUT_DIR/style4s/main.css`. Thie can be overrideen with `using_out_file(...)`.
//! ```
//! use style4rs::Builder;
//! use std::path::Path;
//! Builder::new()
//!    .using_in_folder(Path::new("input_path"))
//!    .using_out_file(Path::new("../../target/style4rs-example/file.css"))
//!    .build()
//!    .ok();
//! ```
//!

pub use style4rs_macros::{style, style_str, style_sheet, style_sheet_str};
pub use style4rs_builder::builder::Builder;
