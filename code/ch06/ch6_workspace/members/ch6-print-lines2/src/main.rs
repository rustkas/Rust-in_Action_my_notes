use std::{borrow::Cow, ffi::{c_char, CStr}};

static B: [u8;10] = [99,97,114,114,121,116,111,119,101,108];
static C: [u8;11] = [116,104,97,110,107,115,102,105,115,104,0];
fn main() {
    let a = 42;
    let b: String;
    let c:Cow<str>;
    #[allow(unused_unsafe)]
    unsafe
    {
        let boxed_bytes_array = Box::new(B);
        let b_ptr = Box::leak(boxed_bytes_array).as_mut_ptr();
        let byte_array_length = B.len();
        let byte_array_capacity = byte_array_length;
        b = String::from_raw_parts(b_ptr, byte_array_length, byte_array_capacity);
        println!("b pointer = {b_ptr:p}");
        let c_ptr = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    println!("a: {a}");
    println!("a: {a}, b: {b}");
    println!("a: {a}, b: {b}, c: {c}");
}
