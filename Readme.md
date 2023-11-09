# Unsafe travels

This project shows how to parse a Rust file and see where the unsafe code is located.
One possible use of that could be combining that data with a Git diff to see if
the unsafe code regions were changed or introduced.

For an example, run

```sh
cargo r -p find_unsafe_ranges examples/unsafe_example/src/main.rs
```
