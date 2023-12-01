# style4rs

* Scoped CSS for Rust web frameworks like Leptos.
* `style!` macro is for writing css inside rust functions directly.

## Background

This crate extracts many ideas from the great work completed by [Abishek P](https://github.com/abishekatp) in his [stylers](https://github.com/abishekatp/stylers) repository.

This is **Work In Progress** and, at this stage, not even for development consumption, let alone production.

It isn't intended to compete with / replace [stylers](https://github.com/abishekatp/stylers), but [stylers](https://github.com/abishekatp/stylers) has a reliance on the `nightly` build which bugged me and I wanted to see if an alternate approach is feasible.

|                       | style4rs |   stylers   |
|-----------------------|:--------:|:-----------:|
| Rust build            | Stable ✓ |  Nightly ☹  |
| style!                |    ✓     |      ✓      |
| style_sheet!          |    -     |      ✓      |
| style_str!            |    -     |      ✓      |
| style_sheet_str!      |    -     |      ✓      |
| css validation        |  ✓ [1]   |    ✓ [2]    |
| custom pseudo classes |    -     |      ✓      |
| Release version       |   none   | 1.0.0-alpha |

[1] Syntactic
[2] Semantic

## Development

`nix develop --impure` will set up an appropriate [Rust](https://www.rust-lang.org/) development environment, with Vscode.

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

```rust
use style4rs_builder::*;

fn main() {
    Style4rsBuilder::build().ok();
}
```