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

impl Default for File {
    fn default() -> Self {
        Self {
            name: String::from(""),
            data: Vec::new(),
        }
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            ..Default::default()
        }
    }

    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        // <1> This method has snuck in to deal with cases where we want to simulate cases where a file has pre-existing data
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    /// Return the "number of bytes read"
    pub fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone(); // <5> Make a copy of the data here, as `save_to.append()` will shrink the input vec
        let read_length = tmp.len();
        save_to.reserve(read_length); // <6> Not strictly necessary, but useful to know about
        save_to.append(&mut tmp); // <7> Allocate sufficient data in the `save_to` buffer to hold the contents of `f`
        read_length
    }
}

pub fn open(_f: &mut File) -> bool {
    true // <3> let's assume for the moment that this always succeeds
}

pub fn close(_f: &mut File) -> bool {
    true // <3>
}
