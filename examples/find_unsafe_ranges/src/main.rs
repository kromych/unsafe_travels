fn main() {
    // Get the file name from the command line arguments
    let file_name = std::env::args().nth(1).expect("expected file name");
    // Get the contents of the file
    let rust_code = std::fs::read_to_string(&file_name).expect("failed to read file");

    // Get the unsafe ranges from the file
    match unsafe_travels::unsafe_ranges(&rust_code) {
        Ok(unsafe_ranges) => {
            // Print the unsafe ranges
            println!("{:#?}", unsafe_ranges);
        }
        Err(e) => {
            // Print the error
            eprintln!(
                "ERROR {} at {:?}..{:?}",
                e,
                e.span().start(),
                e.span().end()
            );
        }
    }

    // An example of unsafe code
    let mut _u = 1;
    unsafe {
        _u = std::mem::transmute(_u);
    }
}
