//! # Builder
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
//!    .using_out_file(Path::new("output_path/target_file.css"))
//!    .build()
//!    .ok();
//! ```


pub mod builder;
mod error;
mod file_visitor;
mod result;
