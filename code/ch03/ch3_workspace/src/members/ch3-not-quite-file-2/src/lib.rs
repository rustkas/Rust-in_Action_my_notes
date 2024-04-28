use std::fmt;

#[derive(Debug)] // <2> This enables `File` to work with `println!` and its `fmt!` sibling macros, used at the bottom of the code listing
pub struct File {
  pub name: String,
  pub data: Vec<u8>,
}

impl fmt::Display for File {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(
          f,
          "File: {}
Data Length: {} bytes
",
          self.name,
        
          self.data.len()
      )
  }
}

pub fn open(_f: &mut File) -> bool {
  true // <3> let's assume for the moment that this always succeeds
}

pub fn close(_f: &mut File) -> bool {
  true // <3>
}



/// Return the "number of bytes read"
pub fn read(f: &File, save_to: &mut Vec<u8>) -> usize { 
let mut tmp = f.data.clone(); // <5> Make a copy of the data here, as `save_to.append()` will shrink the input vec
let read_length = tmp.len();
save_to.reserve(read_length); // <6> Not strictly necessary, but useful to know about
save_to.append(&mut tmp); // <7> Allocate sufficient data in the `save_to` buffer to hold the contents of `f`
read_length
}
