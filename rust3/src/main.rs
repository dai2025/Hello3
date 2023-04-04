use std::sys::unix::weak::{fetch, Weak};

fn main() {
    // Try to load the getpid symbol
    let getpid: Option<Weak<unsafe extern "C" fn() -> libc::pid_t>> = fetch("getpid");

    // Check if the symbol was loaded successfully
    if let Some(getpid) = getpid {
        // Convert the Weak pointer to a reference
        let getpid = getpid.upgrade().unwrap();

        // Call the function and print the result
        let pid = unsafe { getpid() };
        println!("Process ID: {}", pid);
    } else {
        // Handle the error
        println!("Failed to load getpid");
    }
}
