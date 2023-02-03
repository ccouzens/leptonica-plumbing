# leptonica-plumbing

Crate to expose a safe version of the
[leptonica-sys](https://crates.io/crates/leptonica-sys) api.

This is designed to expose the C API of [leptonica](http://www.leptonica.org/)
in a safe manner.

Adding value by deviating from the API is a non-goal. That is left to libraries
that build on top of `leptonica-plumbing`.

## Motivation

I'm a maintainer of both [leptess](https://crates.io/crates/leptess) and
[tesseract-rs](https://crates.io/crates/tesseract).

I noticed that there was a lot of duplication in how they interact with both
[leptonica-sys](https://crates.io/crates/leptonica-sys) and
[tesseract-sys](https://crates.io/crates/tesseract-sys). Having multiple layers
of abstraction in `leptess` and `tesseract-rs` made it hard to reason about
their memory safety.

Having a safety layer that stays simple improves the correctness and
maintainability of the above libraries.

## Testing

To test for memory leaks, test with `valgrind`.

```bash
cargo test --release && valgrind --leak-check=yes --error-exitcode=1 --leak-check=full --show-leak-kinds=all "$(find target/*/deps/ -executable -name 'leptonica_plumbing-*')"
```

You may find that leptonica always leaks 16B of memory.

To test with a manually compiled Leptonica, test with additional environment
variables

```bash
LD_LIBRARY_PATH="$(pwd)/../../DanBloomberg/leptonica/local/lib" PKG_CONFIG_PATH="$(pwd)/../../DanBloomberg/leptonica/local/lib/pkgconfig" cargo test
```

The two can be combined

```bash
LD_LIBRARY_PATH="$(pwd)/../../DanBloomberg/leptonica/local/lib" PKG_CONFIG_PATH="$(pwd)/../../DanBloomberg/leptonica/local/lib/pkgconfig" bash -c 'cargo test --release && valgrind --leak-check=yes --error-exitcode=1 --leak-check=full --show-leak-kinds=all "$(find target/*/deps/ -executable -name 'leptonica_plumbing-*')"'
```
