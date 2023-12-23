# style4rs

* Scoped CSS for Rust web frameworks like Leptos.
* `style!` macro is for writing css inside rust functions directly.
* `style_str!` macro is same as style! macro but returns the tuple (class_name, style_val).

## Acknowledgement

This crate stems from [Abishek P](https://github.com/abishekatp)'s [stylers](https://github.com/abishekatp/stylers) repository, without which this crate would never have been possible.


## Background

This is **Work In Progress** and, at this stage, not even for development consumption, let alone production.

Tests in this crate are derived from [stylers](https://github.com/abishekatp/stylers) (and modified to reflect CSS constructs supported differently here). The core code has been re-done using the [LightningCSS Parser](https://lightningcss.dev/).

It isn't intended to compete with / replace [stylers](https://github.com/abishekatp/stylers), but [stylers](https://github.com/abishekatp/stylers) has a reliance on the `nightly` build which bugged me and I wanted to see if an alternate approach was feasible.

|                           | style4rs       | stylers     |
|---------------------------|:---------------|:------------|
| Rust build                | Stable ✓       | Nightly ☹   |
| style!                    | ✓              | ✓           |
| style_sheet!              | -              | ✓           |
| style_str!                | ✓              | ✓           |
| style_sheet_str!          | -              | ✓           |
| css validation            | ✓              | ✓           |
| minified css              | ✓              | -           |
| custom raw_str function   | - [1]              | ✓           |
| __Specific CSS handling__ |                |             |
| ::deep                    | Passed-through | Handled     |
| @document                 | Passed-through | Handled     |
| __Release__               |                |             |
| Release version           | none           | 1.0.0-alpha |

[1] A consequence of this is not all valid CSS content is parsable if it conflicts with the rust pre-parsing. E.g. `content: "\hello"` results in compile error, whereas `content: "\\hello"` results in css with `\\` rather than the _rust escaped_ `\`. Unicode escape sequences, such as `content: "\01F44D"` appear ok.

## Development

[Nix](https://nixos.org/) can be used to set up a development environment.

Running `> nix develop --impure` will set up [Rust](https://www.rust-lang.org/) with Vscode.

## Usage

`style!` macros will return a deterministic class name to be used by `leptos`.

`build.rs` will transform all `style!` css to the project's `$OUTDIR/style4rs/main.css`.

### Cargo.toml

```toml
[package]
name = "style4rs-test"
version = "0.1.2"
edition = "2021"

[dependencies]
style4rs = { version = "*" }

[build-dependencies]
style4rs-builder = { version = "*" }
```

### In code

```rust
use style4rs::style;

pub fn the_class_name() -> String {
    let class_name = style!{
        #one1{
            color: blue 6px;
        }
        div.one{
            color: red;
            content: "hello";
            font: "1.3em/1.2" Arial, Helvetica, sans-serif;
        }
        wibble {
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
        }
        h2 {
            color: purple;
        }
        @media only screen and (max-width: 1000px) {
            h3 {
                background-color: lightblue;
                color: blue
            }
        }
    };
    class_name.to_string()
}
```

### build.rs

See [Build Scripts - The Cargo Book](https://doc.rust-lang.org/cargo/reference/build-scripts.html).

```rust
use style4rs_builder::*;

fn main() {
    Style4rsBuilder::build().ok();
}
```