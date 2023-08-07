# Learning Rust

Documenting the journey.

## Quick reference

[The Rust book](https://doc.rust-lang.org/book/)

[CLI apps book](https://rust-cli.github.io/book/index.html)

### Creating, compiling and running projects

To create a new project (inside existing project folder):

`cargo init --name "<project name>"` (omit `--name` flag to use folder name)

To run `main.rs` file:

`cargo run`

To build executable binary file:

`rustc main.rs`

### Common syntax

`println!("")` to print line of text. Use `{}` to parse variables and `{:?}` to parse more complex data types.

All `main.rs` files need a `fn main() {}` function.
