use ch3_error_1::{read, File, ERROR};

fn main() {
    let f = File;
    let buffer = vec![];

    read(&f, buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }
}