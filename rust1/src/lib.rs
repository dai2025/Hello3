use std::io::{self, Write};

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "Adding {} and {}", a, b).unwrap();
    let result = a + b;
    writeln!(handle, "Result: {}", result).unwrap();
    result
}

#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn sub2() -> i32 {
    -1
}


