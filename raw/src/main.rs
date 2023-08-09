// smart pointer type that reads from its pointer location without needing to copy it first
use std::borrow::Cow;
// CStr is C-like string type that allows for reading zero-terminated strings
use std::ffi::CStr;
// type alias for Rust's i8 type
// possibility of a platform-specific nuances
use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a = 42;

    // String is a smart pointer type that holds a pointer to the buffer array and a field to store
    // its size
    let b: String;
    let c: Cow<str>;

    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);
        let c_ptr = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    println!("a: {}, b: {}, c: {}", a, b, c);
}
