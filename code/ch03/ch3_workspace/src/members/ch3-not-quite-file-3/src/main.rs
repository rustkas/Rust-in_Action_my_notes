use ch3_not_quite_file_3::{close, open, File};

fn main() {
    let f3_data: Vec<u8> = vec![114, 117, 115, 116, 33]; // <4> An explicit type needs to be provided, as `vec!` can't infer the necessary type through the function boundary
    let mut f3 = File::new_with_data("3.txt", &f3_data); 

    println!("File is \n{f3}");

    let mut buffer: Vec<u8> = vec![];

    if !open(&mut f3) {
        panic!("File is not open");
    }
    
    let f3_length = f3.read(&mut buffer); // <5> Here is the the change in the calling code 
    println!("buffer: {buffer:?}. Read length = {f3_length}");
    
    if !close(&mut f3) {
        panic!("File is not closed");
    }

    let text = String::from_utf8_lossy(&buffer);
    println!("Sequece of numbers as a text is \"{text}\"");
}
