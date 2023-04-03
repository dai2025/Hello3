use std::os::raw::c_int;

extern "C" {
    fn add_numbers(a: c_int, b: c_int) -> c_int;
}

fn main() {
    let a = 10;
    let b = 20;
    let sum = unsafe { add_numbers(a, b) };
    println!("{} + {} = {}", a, b, sum);
}
