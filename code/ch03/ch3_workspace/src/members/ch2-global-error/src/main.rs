use ch2_global_error::{read, File, ERROR};

fn main() {
    let f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);
    unsafe { // <8>
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }
}