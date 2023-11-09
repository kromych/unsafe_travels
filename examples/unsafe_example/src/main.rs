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
