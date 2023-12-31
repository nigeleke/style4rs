//! # Style4rs
//!
//! Style4rs allows [Leptos](https://leptos.dev/) and [Yew](https://yew.rs/) components to encapsulate their own
//! css without encroaching on css defined for other components.
//!
//! It comprises two main areas of functionality; the [macros](../style4rs_macros/#) to enable unique component
//! classes to be defined, and the [builder](../style4rs_builder/#) which merges the components' unique css into
//! a single file.
//!

pub use style4rs_macros::{style, style_str, style_sheet, style_sheet_str};
pub use style4rs_builder::builder::Builder;

