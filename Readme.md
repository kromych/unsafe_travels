# Unsafe travels

This project shows how to parse a Rust file and see where the unsafe code is located.
One possible use of that could be combining that data with a Git diff to see if
the unsafe code regions were changed or introduced.

Currently uses a [fork](https://github.com/kromych/syn.git) of the
[`syn`](https://github.com/dtolnay/syn) crate that just plumbs through the `span-locations`
feature of the `proc-macro2` crate.

For an example, run

```sh
cargo r -p find_unsafe_ranges examples/find_unsafe_ranges/src/main.rs  
```
