use std::fmt;

type FileMode = &'static str;

pub const OPEN: FileMode = "open";
pub const CLOSED: FileMode = "closed";


#[derive(Debug)] // <2> This enables `File` to work with `println!` and its `fmt!` sibling macros, used at the bottom of the code listing
pub struct File {
    pub name: String,
    pub data: Vec<u8>,
    pub state: &'static str,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "File: {}
File mode: {}            
Data Length: {} bytes
",
            self.name,
            self.state,
            self.data.len()
        )
    }
}

impl Default for File {
    fn default() -> Self {
        Self {
            name: String::from(""),
            state: CLOSED,
            data: Vec::new(),
        }
    }
}
pub type ReadLength = usize;

pub trait Read {          
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            ..Default::default()
        }
    }

    pub fn new_with_data(name: &str, data: &Vec<u8>, state: FileMode) -> File {
        // <1> This method has snuck in to deal with cases where we want to simulate cases where a file has pre-existing data
        let mut f = File::new(name);
        f.data = data.clone();
        f.state = state;
        f
    }

    // /// Return the "number of bytes read"
    // pub fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
    //     let mut tmp = self.data.clone(); // <5> Make a copy of the data here, as `save_to.append()` will shrink the input vec
    //     let read_length = tmp.len();
    //     save_to.reserve(read_length); // <6> Not strictly necessary, but useful to know about
    //     save_to.append(&mut tmp); // <7> Allocate sufficient data in the `save_to` buffer to hold the contents of `f`
    //     read_length
    // }
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<ReadLength, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

pub fn open(f: &mut File) -> bool {
    f.state = OPEN;
    true // <3> let's assume for the moment that this always succeeds
}

pub fn close(f: &mut File) -> bool {
    f.state = CLOSED;
    true // <3>
}
