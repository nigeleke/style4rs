# style4rs

[![MIT License](https://img.shields.io/github/license/nigeleke/style4rs?style=plastic)](https://github.com/nigeleke/style4rs/blob/main/LICENCE.md)
[![Language](https://img.shields.io/badge/language-Rust-blue.svg?style=plastic)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/style4rs/acceptance.yml?style=plastic)](https://github.com/nigeleke/style4rs/actions/workflows/acceptance.yml)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/style4rs?style=plastic)](https://codecov.io/gh/nigeleke/style4rs)
![Version](https://img.shields.io/github/v/tag/nigeleke/style4rs?style=plastic)

* Scoped CSS for Rust web frameworks like Leptos.
  
| Macro              | Description                                                                       |
|--------------------|-----------------------------------------------------------------------------------|
| `style!`           | Enable writing css inside rust functions directly.                                |
| `style_str!`       | Same as `style!` macro but returns the tuple (class_name, style_val).             |
| `style_sheet!`     | Extract css from an external css file and importing that into the rust functions. |
| `style_sheet_str!` | Same as `style_sheet!` macro but returns the tuple (class_name, style_val).       |

## Acknowledgement

This crate owes its existence to the [stylers](https://github.com/abishekatp/stylers) repository created by [Abishek P](https://github.com/abishekatp), without which this crate would never have been possible.

## Background

The reason for this crate's development was because [stylers](https://github.com/abishekatp/stylers) has a reliance on the [Rust](https://www.rust-lang.org/) `nightly` build. This bugged me and I wanted to see if an alternate approach was feasible.

## Documentation

* [Site](https://nigeleke.github.io/style4rs)
* [GitHub](https://github.com/nigeleke/style4rs)
* [Crates](https://nigeleke.github.io/style4rs/core/index.html)
* [Coverage Report](https://nigeleke.github.io/style4rs/coverage/index.html)

## Alternatives

If you're choosing between [style4rs](https://nigeleke.github.io/style4rs/) & [stylers](https://github.com/abishekatp/stylers), the following comparision may help. Also, an additional crate [styled](https://docs.rs/styled/latest/styled/) exists, which requires [stylist](https://docs.rs/stylist/latest/stylist/). At the time of writing[0] [stylist](https://docs.rs/stylist/latest/stylist/) depends on the nightly build too; they haven't been analysed further.

|                           | style4rs         | stylers     | styled / stylist | Comments |
|---------------------------|------------------|-------------|------------------|----------|
| Rust build                | Stable ✓         | Nightly ☹   | Nightly ☹        |          |
| Minified CSS              | ✓                | x           | ?                |          |
| __macros__                |                  |             |                  |          |
| style!                    | ✓                | ✓           | ✓                |          |
| style_sheet!              | ✓                | ✓           | ?                |          |
| style_str!                | ✓                | ✓           | ?                |          |
| style_sheet_str!          | ✓                | ✓           | ?                |          |
| css validation            | ✓                | ✓+          | ?                | [1]      |
| __Misc__                  |                  |             |                  |          |
| custom `raw_str` function | x                | ✓           | ?                | [2]      |
| __Specific CSS handling__ |                  |             |                  |          |
| ::deep                    | Passed-through   | Handled     | ?                | [3]      |
| @document                 | Passed-through   | Handled     | ?                | [3]      |
| __Released ?__            |                  |             |                  |          |
| Release version           | Not in crates.io | 1.0.0-alpha | 0.2.0 / 0.13.0   |          |

   0. 29-Dec-2023
   1. `style4rs` highlights syntactic errors around the entire CSS block (with an error message described by [lightningcss](https://lightningcss.dev/)).<br/>`stylers` highlights errors at their precise line and also provides semantic checks / hints.                                                           
   2. A consequence of not supporting a `raw_str` function is not all valid CSS content is parsable if it conflicts with the rust parsing. E.g. `content: "\hello"` results in compile error, whereas `content: "\\hello"` results in css with `\\` rather than the _rust escaped_ `\`. Unicode escape sequences, such as `content: "\01F44D"` appear ok.
   3. The best approach for handling these is to be determined. At this stage, my other projects are unlikely to require this CSS. Feel free to raise an issue / use-case if deemed required.

## Development

[Nix](https://nixos.org/) can be used to set up a development environment.

Running `> nix develop --impure` will set up [Rust](https://www.rust-lang.org/) with Vscode.

The core code uses the [LightningCSS Parser](https://lightningcss.dev/).

Tests in this crate are derived from [stylers](https://github.com/abishekatp/stylers) (and modified to reflect CSS constructs supported differently here).


## Usage

See `style4rs-macros/tests/macro_foobar_spec` and `style4rs-test/build.rs`.

`Style4rsBuilder::build()` transforms the __style4rs__ macros to the project's `$OUTDIR/style4rs/main.css`.

### Cargo.toml

```toml
[package]
name = "style4rs-test"
version = "0.1.0"
edition = "2021"

[dependencies]
style4rs = { version = "*" }

[build-dependencies]
style4rs = { version = "*" }
```

### build.rs

See [Build Scripts - The Cargo Book](https://doc.rust-lang.org/cargo/reference/build-scripts.html).

```rust
use style4rs::Style4rsBuilder;

fn main() {
    Style4rsBuilder::build().ok();
}
```
