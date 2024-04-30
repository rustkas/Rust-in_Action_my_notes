fn main() {
    let a = 42;
    let hello = "hello";
    let as_bytes = hello.as_bytes();
    println!("string = {} as bytes {as_bytes:?}", stringify!(hello));

    let bytes_array = [104, 101, 108, 108, 111]; // Пример массива байтов

    let string = match std::str::from_utf8(&bytes_array) {
        Ok(s) => s,
        Err(e) => panic!("Ошибка при разборе байтов: {:?}", e),
    };

    println!("string = {string}");
    let b: String;
    #[allow(unused_unsafe)]
    unsafe
    {
        let boxed_bytes_array = Box::new(bytes_array);
        let b_ptr = Box::leak(boxed_bytes_array).as_mut_ptr();
        let byte_array_length = bytes_array.len();
        let byte_array_capacity = byte_array_length;
        b = String::from_raw_parts(b_ptr, byte_array_length, byte_array_capacity);
        println!("b pointer = {b_ptr:p}");
    }

    println!("a: {a}");
    println!("a: {a}, b: {b}");
}
