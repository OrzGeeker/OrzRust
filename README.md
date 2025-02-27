# OrzRust

The Rust Project Template for OrzGeeker

## Code Format With `rustfmt`

```bash
rustup component add rustfmt  # install rustfmt
cargo fmt                     # format current caret
```

## Code Fix with `rustfix`

```bash
cargo fix  # fix code as suggestion
```

## Code Lint with `clippy`

```bash
rustup component add clippy  # install clippy
cargo clippy                 # lint current caret
```

## Code in VSC with [`rust-analyzer`][rust-analyzer]


[rust-analyzer]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

## Use Nightly Rust

```bash
rustup toolchain list                # list nightly rust
rustup toolchain install nightly     # install nightly rust
rustup override set nightly          # use nightly rust in local dir
```

## [The Rust Workflow][rust-workflow]

[rust-workflow]: https://doc.rust-lang.org/book/appendix-07-nightly-rust.html

💡 we can learn rust workflow, and apply on our own project.

🚊 6-weeks train model

```
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

- [All Rust Keywords](https://doc.rust-lang.org/book/appendix-01-keywords.html)
- [All Rust Operators & Symbols](https://doc.rust-lang.org/book/appendix-02-operators.html)
- [Derivable Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
- [Write CLI in Rust](https://rust-cli.github.io/book/tutorial/packaging.html)