pub type File = String; // <2> Create a type alias. The compiler won't distinguish between String & File, but your source code will.

pub fn open(_f: &mut File) -> bool {
  true // <3> let's assume for the moment that this always succeeds
}

pub fn close(_f: &mut File) -> bool {
  true // <3>
}

pub fn read(_f: &mut File, _save_to: &mut Vec<u8>) -> ! { // <5> Using `!` as a return type indicates to the Rust compiler that this function never returns
  unimplemented!() // <6> A macro that crashes the program if it is encountered
}
