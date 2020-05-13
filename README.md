# simple-git-diff

simple-git-diff simplifies diff output for git on the command line. Inspired by
[diff-so-fancy](https://github.com/so-fancy/diff-so-fancy).

## Installation

Currently, pre-compiled binaries of simple-git-diff aren't being distributed.
You can install it with
[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) by
running

```
cargo install --git https://github.com/rsookram/simple-git-diff
```

## Building

simple-git-diff can be built from source by cloning this repository and using
Cargo.

```
git clone https://github.com/rsookram/simple-git-diff
cd simple-git-diff
cargo build --release
```

## Dependencies

The [`term_size`](https://crates.io/crates/term_size) crate is used when
drawing horizontal dividers which span the width of the terminal.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.
