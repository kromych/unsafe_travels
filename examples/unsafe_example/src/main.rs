//! An example of unsafe code

fn main() {
    let mut _u = 1;
    unsafe {
        _u = std::mem::transmute(_u);
    }
}
