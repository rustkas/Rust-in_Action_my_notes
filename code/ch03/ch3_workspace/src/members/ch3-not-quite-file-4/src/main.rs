use ch3_not_quite_file_4::{close, open, File, Read, ReadLength, CLOSED, OPEN};

fn main() {
    let mut f4 = File::new_with_data(
        "4.txt", // <3>
        &vec![114, 117, 115, 116, 33],
        CLOSED,
    );
    println!("File is \n{f4}");
    if !open(&mut f4) && f4.state == OPEN {
        panic!("File is not open");
    }
    let mut buffer: Vec<u8> = vec![];
    let f3_length:ReadLength = f4.read(&mut buffer).unwrap(); // <5> Here is the the change in the calling code
    println!("buffer: {buffer:?}. Read length = {f3_length}");

    if !close(&mut f4) && f4.state == CLOSED {
        panic!("File is not closed");
    }

    let text = String::from_utf8_lossy(&buffer);
    println!("Sequece of numbers as a text is \"{text}\"");
}
