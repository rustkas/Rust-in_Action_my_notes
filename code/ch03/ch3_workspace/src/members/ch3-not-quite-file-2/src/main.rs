use ch3_not_quite_file_2::{close, open, read, File};

fn main() {
    let mut f2 = File {
        name: String::from("2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);
    println!("File is {f2}");

    let f2_length = read(&f2, &mut buffer);
    println!("buffer: {buffer:?}. Read length = {f2_length}");
    close(&mut f2);

  
    {
        let sparkle_heart = vec![240, 159, 146, 150];

        let sparkle_heart = String::from_utf8_lossy(&sparkle_heart);

        assert_eq!("💖", sparkle_heart);
    }
  let text = String::from_utf8_lossy(&buffer); // <9> Convert `Vec<u8>` to `String`. Any bytes that are not valid UTF-8 are replaced with

    println!("Sequece of numbers as a text is \"{text}\"") // <10>  View [114, 117, 115, 116, 33] as an actual word
}
