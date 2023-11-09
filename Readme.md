# Unsafe travels

This project shows how to parse a Rust file and see where the unsafe code is located.
One possible use of that could be combining that data with a Git diff to see if
the unsafe code regions were changed or introduced.

For an example, run

```sh
cargo r -p find_unsafe_ranges examples/unsafe_example/src/main.rs
```

which produces for

```rust
//! An example of unsafe code

struct _S {
    _a: u32,
}

unsafe impl Send for _S {}

unsafe fn mute() {
    let _u = 1;
    unsafe {
        let _s: _S = std::mem::transmute(_u);
    }
}

fn main() {
    unsafe {
        mute();
    }
}
```

```
LineColumn { line: 7, column: 0 }..LineColumn { line: 7, column: 26 }
LineColumn { line: 9, column: 0 }..LineColumn { line: 14, column: 1 }
LineColumn { line: 11, column: 4 }..LineColumn { line: 13, column: 5 }
LineColumn { line: 17, column: 4 }..LineColumn { line: 19, column: 5 }
```
