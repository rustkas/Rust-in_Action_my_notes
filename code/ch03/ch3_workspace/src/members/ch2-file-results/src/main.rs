use ch2_file_results::{close, open, File};



fn main() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new_with_data("4.txt", &f4_data);

    let mut buffer: Vec<u8> = vec![];
    f4 = open(f4).unwrap();    
    let f4_length = f4.read(&mut buffer).unwrap();

    f4 = close(f4).unwrap();  

    let text = String::from_utf8_lossy(&buffer);
    
    println!("{f4:?}");
    println!("{} is {} bytes long", &f4.name, f4_length);
    println!("{}", text);
}
