# Prerequisites

- [cargo-make](https://crates.io/crates/cargo-make)
- [wasm-bindgen-cli](https://rustwasm.github.io/docs/wasm-bindgen/reference/cli.html)
- wasm target: `rustup target add wasm32-unknown-unknown`

# Crate overview

## page

This crate holds the wasm code.

## maker

This crate processes the page and presents it in its `www` directory.

# Making

We need to use a build system while waiting on [issue 545](https://github.com/rust-lang/cargo/issues/545).

1. Install

2. Run the make script:

`cargo make`
