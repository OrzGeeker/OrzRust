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