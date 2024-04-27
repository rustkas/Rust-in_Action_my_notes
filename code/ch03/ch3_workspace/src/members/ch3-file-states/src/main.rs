use ch3_file_states::{close, File};

fn main() {
    let mut f5 = File::new("5.txt"); 

  let mut buffer: Vec<u8> = vec![];

  if f5.read(&mut buffer).is_err() {
    println!("Error checking is working");
  }

  f5 = ch3_file_states::open(f5).unwrap();                        // <9>
  let f5_length = f5.read(&mut buffer).unwrap(); // <9>
  f5 = close(f5).unwrap();                       // <9>
  
  let text = String::from_utf8_lossy(&buffer);

  println!("{f5}");
  println!("{}", text);
  println!("File length: {f5_length}");
}
